use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{Claims, CreateTripRequest, Trip, TripListResponse, TripResponse, TripSearchParams};
use crate::services::AuthService;

/// GET /api/trips - Öffentliche Reisenliste
pub async fn list_trips(
    pool: web::Data<MySqlPool>,
    query: web::Query<TripSearchParams>,
) -> AppResult<HttpResponse> {
    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(10).min(50);
    let offset = (page - 1) * per_page;

    // Nur aktive Reisen anzeigen
    let trips: Vec<Trip> = sqlx::query_as(
        r#"
        SELECT * FROM trips 
        WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()
        ORDER BY abfahrt_datum ASC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM trips WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()"
    )
    .fetch_one(pool.get_ref())
    .await?;

    let trips_response: Vec<TripResponse> = trips.into_iter().map(TripResponse::from).collect();

    Ok(HttpResponse::Ok().json(TripListResponse {
        reisen: trips_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// GET /api/trips/search - Reisen suchen mit Filtern
pub async fn search_trips(
    pool: web::Data<MySqlPool>,
    query: web::Query<TripSearchParams>,
) -> AppResult<HttpResponse> {
    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(10).min(50);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        "SELECT * FROM trips WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()"
    );
    let mut count_sql = String::from(
        "SELECT COUNT(*) FROM trips WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()"
    );

    // Filter bauen
    if let Some(ref ziel) = query.ziel {
        let filter = format!(" AND ziel LIKE '%{}%'", ziel.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref von) = query.abfahrt_von {
        let filter = format!(" AND abfahrt_datum >= '{}'", von);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref bis) = query.abfahrt_bis {
        let filter = format!(" AND abfahrt_datum <= '{}'", bis);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(min) = query.preis_min {
        let filter = format!(" AND preis_pro_person >= {}", min);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(max) = query.preis_max {
        let filter = format!(" AND preis_pro_person <= {}", max);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if query.nur_verfuegbar.unwrap_or(false) {
        let filter = " AND aktuelle_buchungen < max_teilnehmer";
        sql.push_str(filter);
        count_sql.push_str(filter);
    }

    // Sortierung
    let order = match query.sortierung.as_deref() {
        Some("preis_asc") => " ORDER BY preis_pro_person ASC",
        Some("preis_desc") => " ORDER BY preis_pro_person DESC",
        Some("datum_desc") => " ORDER BY abfahrt_datum DESC",
        _ => " ORDER BY abfahrt_datum ASC",
    };
    sql.push_str(order);
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let trips: Vec<Trip> = sqlx::query_as(&sql)
        .fetch_all(pool.get_ref())
        .await?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(pool.get_ref())
        .await?;

    let trips_response: Vec<TripResponse> = trips.into_iter().map(TripResponse::from).collect();

    Ok(HttpResponse::Ok().json(TripListResponse {
        reisen: trips_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// GET /api/trips/{id} - Einzelne Reise abrufen
pub async fn get_trip(
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let trip_id = path.into_inner();

    let trip: Trip = sqlx::query_as("SELECT * FROM trips WHERE id = ?")
        .bind(&trip_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Reise nicht gefunden".to_string()))?;

    Ok(HttpResponse::Ok().json(TripResponse::from(trip)))
}

/// GET /api/admin/trips - Admin: Alle Reisen auflisten
pub async fn admin_list_trips(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    query: web::Query<TripSearchParams>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let trips: Vec<Trip> = sqlx::query_as(
        "SELECT * FROM trips ORDER BY erstellt_am DESC LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trips")
        .fetch_one(pool.get_ref())
        .await?;

    let trips_response: Vec<TripResponse> = trips.into_iter().map(TripResponse::from).collect();

    Ok(HttpResponse::Ok().json(TripListResponse {
        reisen: trips_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// POST /api/admin/trips - Admin: Neue Reise erstellen
pub async fn create_trip(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<CreateTripRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Validierung: Rückkehrdatum muss nach Abfahrtsdatum sein
    if body.rueckkehr_datum < body.abfahrt_datum {
        return Err(AppError::ValidationError(
            "Rückkehrdatum muss nach dem Abfahrtsdatum liegen".to_string(),
        ));
    }

    let trip_id = Uuid::new_v4().to_string();
    let status = body.status.clone().unwrap_or_else(|| "geplant".to_string());
    let bilder = body.bilder.as_ref().map(|b| serde_json::to_string(b).unwrap_or_default());
    let highlights = body.highlights.as_ref().map(|h| serde_json::to_string(h).unwrap_or_default());
    let inkludiert = body.inkludiert.as_ref().map(|i| serde_json::to_string(i).unwrap_or_default());
    let nicht_inkludiert = body.nicht_inkludiert.as_ref().map(|n| serde_json::to_string(n).unwrap_or_default());

    sqlx::query(
        r#"
        INSERT INTO trips (
            id, titel, beschreibung, ziel, abfahrtsort,
            abfahrt_datum, abfahrt_zeit, rueckkehr_datum, rueckkehr_zeit,
            preis_pro_person, bus_id, max_teilnehmer, status,
            bilder, highlights, inkludiert, nicht_inkludiert
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&trip_id)
    .bind(&body.titel)
    .bind(&body.beschreibung)
    .bind(&body.ziel)
    .bind(&body.abfahrtsort)
    .bind(&body.abfahrt_datum)
    .bind(&body.abfahrt_zeit)
    .bind(&body.rueckkehr_datum)
    .bind(&body.rueckkehr_zeit)
    .bind(&body.preis_pro_person)
    .bind(&body.bus_id)
    .bind(&body.max_teilnehmer)
    .bind(&status)
    .bind(&bilder)
    .bind(&highlights)
    .bind(&inkludiert)
    .bind(&nicht_inkludiert)
    .execute(pool.get_ref())
    .await?;

    let trip: Trip = sqlx::query_as("SELECT * FROM trips WHERE id = ?")
        .bind(&trip_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(TripResponse::from(trip)))
}

/// PUT /api/admin/trips/{id} - Admin: Reise aktualisieren
pub async fn update_trip(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<CreateTripRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let trip_id = path.into_inner();

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Prüfen ob Reise existiert
    let _existing: Trip = sqlx::query_as("SELECT * FROM trips WHERE id = ?")
        .bind(&trip_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Reise nicht gefunden".to_string()))?;

    let status = body.status.clone().unwrap_or_else(|| "geplant".to_string());
    let bilder = body.bilder.as_ref().map(|b| serde_json::to_string(b).unwrap_or_default());
    let highlights = body.highlights.as_ref().map(|h| serde_json::to_string(h).unwrap_or_default());
    let inkludiert = body.inkludiert.as_ref().map(|i| serde_json::to_string(i).unwrap_or_default());
    let nicht_inkludiert = body.nicht_inkludiert.as_ref().map(|n| serde_json::to_string(n).unwrap_or_default());

    sqlx::query(
        r#"
        UPDATE trips SET
            titel = ?, beschreibung = ?, ziel = ?, abfahrtsort = ?,
            abfahrt_datum = ?, abfahrt_zeit = ?, rueckkehr_datum = ?, rueckkehr_zeit = ?,
            preis_pro_person = ?, bus_id = ?, max_teilnehmer = ?, status = ?,
            bilder = ?, highlights = ?, inkludiert = ?, nicht_inkludiert = ?,
            aktualisiert_am = NOW()
        WHERE id = ?
        "#
    )
    .bind(&body.titel)
    .bind(&body.beschreibung)
    .bind(&body.ziel)
    .bind(&body.abfahrtsort)
    .bind(&body.abfahrt_datum)
    .bind(&body.abfahrt_zeit)
    .bind(&body.rueckkehr_datum)
    .bind(&body.rueckkehr_zeit)
    .bind(&body.preis_pro_person)
    .bind(&body.bus_id)
    .bind(&body.max_teilnehmer)
    .bind(&status)
    .bind(&bilder)
    .bind(&highlights)
    .bind(&inkludiert)
    .bind(&nicht_inkludiert)
    .bind(&trip_id)
    .execute(pool.get_ref())
    .await?;

    let trip: Trip = sqlx::query_as("SELECT * FROM trips WHERE id = ?")
        .bind(&trip_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(TripResponse::from(trip)))
}

/// DELETE /api/admin/trips/{id} - Admin: Reise löschen
pub async fn delete_trip(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let trip_id = path.into_inner();

    // Prüfen ob Buchungen existieren
    let booking_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE reise_id = ? AND status != 'storniert'"
    )
    .bind(&trip_id)
    .fetch_one(pool.get_ref())
    .await?;

    if booking_count.0 > 0 {
        return Err(AppError::BadRequest(
            "Reise kann nicht gelöscht werden, da Buchungen existieren".to_string(),
        ));
    }

    sqlx::query("DELETE FROM trips WHERE id = ?")
        .bind(&trip_id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Hilfsfunktion: Admin-Berechtigung prüfen
fn verify_admin(req: &HttpRequest, config: &Config) -> AppResult<Claims> {
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
    let claims = AuthService::validate_token(token, config)?;

    if claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}
