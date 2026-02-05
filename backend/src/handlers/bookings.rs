use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{
    Booking, BookingListResponse, BookingResponse, BookingSearchParams, CancelBookingRequest,
    Claims, CreateBookingRequest, Trip, UpdateBookingRequest, User,
};
use crate::services::{AuthService, EmailService};

/// GET /api/bookings - Eigene Buchungen auflisten
pub async fn list_user_bookings(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    query: web::Query<BookingSearchParams>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;

    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(10).min(50);
    let offset = (page - 1) * per_page;

    let bookings: Vec<Booking> = sqlx::query_as(
        "SELECT * FROM bookings WHERE kunde_id = ? ORDER BY buchungsdatum DESC LIMIT ? OFFSET ?"
    )
    .bind(&claims.sub)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bookings WHERE kunde_id = ?")
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await?;

    let mut bookings_response: Vec<BookingResponse> = Vec::new();
    for booking in bookings {
        let mut response = BookingResponse::from(booking.clone());
        
        // Reise-Infos laden
        if let Ok(trip) = sqlx::query_as::<_, Trip>("SELECT * FROM trips WHERE id = ?")
            .bind(&booking.reise_id)
            .fetch_one(pool.get_ref())
            .await
        {
            response.reise_titel = Some(trip.titel);
            response.reise_ziel = Some(trip.ziel);
            response.reise_datum = Some(format!("{} - {}", trip.abfahrt_datum, trip.rueckkehr_datum));
        }
        
        bookings_response.push(response);
    }

    Ok(HttpResponse::Ok().json(BookingListResponse {
        buchungen: bookings_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// POST /api/bookings - Neue Buchung erstellen
pub async fn create_booking(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    email_service: web::Data<EmailService>,
    body: web::Json<CreateBookingRequest>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Reise laden und validieren
    let trip: Trip = sqlx::query_as("SELECT * FROM trips WHERE id = ?")
        .bind(&body.reise_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Reise nicht gefunden".to_string()))?;

    // Prüfen ob Reise buchbar ist
    if trip.status != "aktiv" {
        return Err(AppError::BadRequest("Diese Reise ist nicht mehr buchbar".to_string()));
    }

    let verfuegbar = trip.max_teilnehmer - trip.aktuelle_buchungen;
    if body.anzahl_personen > verfuegbar {
        return Err(AppError::BadRequest(format!(
            "Nur noch {} Plätze verfügbar",
            verfuegbar
        )));
    }

    // Buchungsnummer generieren
    let year = Utc::now().format("%Y").to_string();
    let booking_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE buchungsnummer LIKE ?"
    )
    .bind(format!("{}-%%", year))
    .fetch_one(pool.get_ref())
    .await?;
    let buchungsnummer = format!("{}-{:04}", year, booking_count.0 + 1);

    // Gesamtpreis berechnen
    let gesamtpreis = trip.preis_pro_person * (body.anzahl_personen as f64);

    let booking_id = Uuid::new_v4().to_string();
    let mitreisende = body.mitreisende.as_ref().map(|m| serde_json::to_string(m).unwrap_or_default());

    // Buchung erstellen
    sqlx::query(
        r#"
        INSERT INTO bookings (
            id, buchungsnummer, kunde_id, reise_id, anzahl_personen,
            gesamtpreis, status, mitreisende, bemerkungen, zahlungsstatus, zahlungsmethode
        ) VALUES (?, ?, ?, ?, ?, ?, 'ausstehend', ?, ?, 'ausstehend', ?)
        "#
    )
    .bind(&booking_id)
    .bind(&buchungsnummer)
    .bind(&claims.sub)
    .bind(&body.reise_id)
    .bind(&body.anzahl_personen)
    .bind(&gesamtpreis)
    .bind(&mitreisende)
    .bind(&body.bemerkungen)
    .bind(&body.zahlungsmethode)
    .execute(pool.get_ref())
    .await?;

    // Buchungszähler der Reise aktualisieren
    sqlx::query("UPDATE trips SET aktuelle_buchungen = aktuelle_buchungen + ? WHERE id = ?")
        .bind(&body.anzahl_personen)
        .bind(&body.reise_id)
        .execute(pool.get_ref())
        .await?;

    // Buchung und Benutzer für E-Mail laden
    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_one(pool.get_ref())
        .await?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await?;

    // Bestätigungs-E-Mail senden
    let email_service = email_service.clone();
    tokio::spawn(async move {
        if let Err(e) = email_service.send_booking_confirmation(&user, &booking, &trip).await {
            tracing::error!("Fehler beim Senden der Buchungsbestätigung: {}", e);
        }
    });

    let mut response = BookingResponse::from(booking);
    response.reise_titel = Some(trip.titel);
    response.reise_ziel = Some(trip.ziel);

    Ok(HttpResponse::Created().json(response))
}

/// GET /api/bookings/{id} - Einzelne Buchung abrufen
pub async fn get_booking(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;
    let booking_id = path.into_inner();

    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Buchung nicht gefunden".to_string()))?;

    // Prüfen ob Buchung dem Benutzer gehört
    if booking.kunde_id != claims.sub && claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }

    let mut response = BookingResponse::from(booking.clone());

    // Reise-Infos laden
    if let Ok(trip) = sqlx::query_as::<_, Trip>("SELECT * FROM trips WHERE id = ?")
        .bind(&booking.reise_id)
        .fetch_one(pool.get_ref())
        .await
    {
        response.reise_titel = Some(trip.titel);
        response.reise_ziel = Some(trip.ziel);
        response.reise_datum = Some(format!("{} - {}", trip.abfahrt_datum, trip.rueckkehr_datum));
    }

    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/bookings/{id}/cancel - Buchung stornieren
pub async fn cancel_booking(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<CancelBookingRequest>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;
    let booking_id = path.into_inner();

    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Buchung nicht gefunden".to_string()))?;

    // Prüfen ob Buchung dem Benutzer gehört
    if booking.kunde_id != claims.sub && claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }

    // Prüfen ob Buchung stornierbar ist
    if booking.status == "storniert" {
        return Err(AppError::BadRequest("Buchung ist bereits storniert".to_string()));
    }

    // Buchung stornieren
    sqlx::query(
        "UPDATE bookings SET status = 'storniert', bemerkungen = CONCAT(COALESCE(bemerkungen, ''), '\n[Storniert] ', ?), aktualisiert_am = NOW() WHERE id = ?"
    )
    .bind(body.grund.as_deref().unwrap_or("Auf Kundenwunsch"))
    .bind(&booking_id)
    .execute(pool.get_ref())
    .await?;

    // Buchungszähler der Reise aktualisieren
    sqlx::query("UPDATE trips SET aktuelle_buchungen = aktuelle_buchungen - ? WHERE id = ?")
        .bind(&booking.anzahl_personen)
        .bind(&booking.reise_id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Buchung erfolgreich storniert"
    })))
}

/// GET /api/admin/bookings - Admin: Alle Buchungen auflisten
pub async fn admin_list_bookings(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    query: web::Query<BookingSearchParams>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM bookings WHERE 1=1");
    let mut count_sql = String::from("SELECT COUNT(*) FROM bookings WHERE 1=1");

    if let Some(ref kunde_id) = query.kunde_id {
        let filter = format!(" AND kunde_id = '{}'", kunde_id.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref reise_id) = query.reise_id {
        let filter = format!(" AND reise_id = '{}'", reise_id.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref status) = query.status {
        let filter = format!(" AND status = '{}'", status.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref buchungsnummer) = query.buchungsnummer {
        let filter = format!(" AND buchungsnummer LIKE '%{}%'", buchungsnummer.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    sql.push_str(&format!(" ORDER BY buchungsdatum DESC LIMIT {} OFFSET {}", per_page, offset));

    let bookings: Vec<Booking> = sqlx::query_as(&sql)
        .fetch_all(pool.get_ref())
        .await?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(pool.get_ref())
        .await?;

    let mut bookings_response: Vec<BookingResponse> = Vec::new();
    for booking in bookings {
        let mut response = BookingResponse::from(booking.clone());
        
        // Kunde-Infos laden
        if let Ok(user) = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(&booking.kunde_id)
            .fetch_one(pool.get_ref())
            .await
        {
            response.kunde_name = Some(format!("{} {}", user.vorname, user.nachname));
            response.kunde_email = Some(user.email);
        }

        // Reise-Infos laden
        if let Ok(trip) = sqlx::query_as::<_, Trip>("SELECT * FROM trips WHERE id = ?")
            .bind(&booking.reise_id)
            .fetch_one(pool.get_ref())
            .await
        {
            response.reise_titel = Some(trip.titel);
            response.reise_ziel = Some(trip.ziel);
            response.reise_datum = Some(format!("{} - {}", trip.abfahrt_datum, trip.rueckkehr_datum));
        }
        
        bookings_response.push(response);
    }

    Ok(HttpResponse::Ok().json(BookingListResponse {
        buchungen: bookings_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// GET /api/admin/bookings/{id} - Admin: Einzelne Buchung
pub async fn admin_get_booking(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let booking_id = path.into_inner();

    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Buchung nicht gefunden".to_string()))?;

    let mut response = BookingResponse::from(booking.clone());

    // Kunde-Infos laden
    if let Ok(user) = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&booking.kunde_id)
        .fetch_one(pool.get_ref())
        .await
    {
        response.kunde_name = Some(format!("{} {}", user.vorname, user.nachname));
        response.kunde_email = Some(user.email);
    }

    // Reise-Infos laden
    if let Ok(trip) = sqlx::query_as::<_, Trip>("SELECT * FROM trips WHERE id = ?")
        .bind(&booking.reise_id)
        .fetch_one(pool.get_ref())
        .await
    {
        response.reise_titel = Some(trip.titel);
        response.reise_ziel = Some(trip.ziel);
        response.reise_datum = Some(format!("{} - {}", trip.abfahrt_datum, trip.rueckkehr_datum));
    }

    Ok(HttpResponse::Ok().json(response))
}

/// PUT /api/admin/bookings/{id} - Admin: Buchung aktualisieren
pub async fn admin_update_booking(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateBookingRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let booking_id = path.into_inner();

    // Prüfen ob Buchung existiert
    let _existing: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Buchung nicht gefunden".to_string()))?;

    // Felder aktualisieren
    if let Some(ref status) = body.status {
        sqlx::query("UPDATE bookings SET status = ?, aktualisiert_am = NOW() WHERE id = ?")
            .bind(status)
            .bind(&booking_id)
            .execute(pool.get_ref())
            .await?;
    }

    if let Some(ref zahlungsstatus) = body.zahlungsstatus {
        sqlx::query("UPDATE bookings SET zahlungsstatus = ?, aktualisiert_am = NOW() WHERE id = ?")
            .bind(zahlungsstatus)
            .bind(&booking_id)
            .execute(pool.get_ref())
            .await?;
    }

    if let Some(ref bemerkungen) = body.bemerkungen {
        sqlx::query("UPDATE bookings SET bemerkungen = ?, aktualisiert_am = NOW() WHERE id = ?")
            .bind(bemerkungen)
            .bind(&booking_id)
            .execute(pool.get_ref())
            .await?;
    }

    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&booking_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(BookingResponse::from(booking)))
}

/// Hilfsfunktion: Claims aus Header extrahieren
fn extract_claims(req: &HttpRequest, config: &Config) -> AppResult<Claims> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    let token = &auth_header[7..];
    AuthService::validate_token(token, config)
}

/// Hilfsfunktion: Admin-Berechtigung prüfen
fn verify_admin(req: &HttpRequest, config: &Config) -> AppResult<Claims> {
    let claims = extract_claims(req, config)?;
    if claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }
    Ok(claims)
}
