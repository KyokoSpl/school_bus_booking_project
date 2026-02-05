use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::Claims;
use crate::services::AuthService;

/// GET /api/admin/dashboard - Dashboard-Statistiken abrufen
pub async fn get_dashboard_stats(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    // Gesamtzahlen
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE rolle = 'kunde'")
        .fetch_one(pool.get_ref())
        .await?;

    let total_trips: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM trips")
        .fetch_one(pool.get_ref())
        .await?;

    let active_trips: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM trips WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()"
    )
    .fetch_one(pool.get_ref())
    .await?;

    let total_bookings: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bookings")
        .fetch_one(pool.get_ref())
        .await?;

    let pending_bookings: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE status = 'ausstehend'"
    )
    .fetch_one(pool.get_ref())
    .await?;

    let confirmed_bookings: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE status IN ('bestaetigt', 'bezahlt')"
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Umsatz
    let total_revenue: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(gesamtpreis) FROM bookings WHERE zahlungsstatus = 'bezahlt'"
    )
    .fetch_one(pool.get_ref())
    .await?;

    let pending_revenue: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(gesamtpreis) FROM bookings WHERE zahlungsstatus = 'ausstehend' AND status != 'storniert'"
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Buchungen diesen Monat
    let monthly_bookings: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE MONTH(buchungsdatum) = MONTH(CURDATE()) AND YEAR(buchungsdatum) = YEAR(CURDATE())"
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Letzte 5 Buchungen
    let recent_bookings: Vec<RecentBooking> = sqlx::query_as(
        r#"
        SELECT 
            b.buchungsnummer,
            CONCAT(u.vorname, ' ', u.nachname) as kunde_name,
            t.titel as reise_titel,
            b.gesamtpreis,
            b.status,
            b.buchungsdatum
        FROM bookings b
        JOIN users u ON b.kunde_id = u.id
        JOIN trips t ON b.reise_id = t.id
        ORDER BY b.buchungsdatum DESC
        LIMIT 5
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    // Kommende Reisen
    let upcoming_trips: Vec<UpcomingTrip> = sqlx::query_as(
        r#"
        SELECT 
            id,
            titel,
            ziel,
            abfahrt_datum,
            max_teilnehmer,
            aktuelle_buchungen,
            (max_teilnehmer - aktuelle_buchungen) as verfuegbar
        FROM trips
        WHERE status = 'aktiv' AND abfahrt_datum >= CURDATE()
        ORDER BY abfahrt_datum ASC
        LIMIT 5
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    // Busse
    let total_buses: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM buses")
        .fetch_one(pool.get_ref())
        .await?;

    let available_buses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM buses WHERE status = 'verfuegbar'"
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(DashboardStats {
        benutzer: UserStats {
            gesamt: total_users.0,
        },
        reisen: TripStats {
            gesamt: total_trips.0,
            aktiv: active_trips.0,
        },
        buchungen: BookingStats {
            gesamt: total_bookings.0,
            ausstehend: pending_bookings.0,
            bestaetigt: confirmed_bookings.0,
            diesen_monat: monthly_bookings.0,
        },
        umsatz: RevenueStats {
            bezahlt: total_revenue.0.unwrap_or(0.0),
            ausstehend: pending_revenue.0.unwrap_or(0.0),
        },
        busse: BusStats {
            gesamt: total_buses.0,
            verfuegbar: available_buses.0,
        },
        letzte_buchungen: recent_bookings,
        kommende_reisen: upcoming_trips,
    }))
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

// Response-Typen

#[derive(Debug, serde::Serialize)]
pub struct DashboardStats {
    pub benutzer: UserStats,
    pub reisen: TripStats,
    pub buchungen: BookingStats,
    pub umsatz: RevenueStats,
    pub busse: BusStats,
    pub letzte_buchungen: Vec<RecentBooking>,
    pub kommende_reisen: Vec<UpcomingTrip>,
}

#[derive(Debug, serde::Serialize)]
pub struct UserStats {
    pub gesamt: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct TripStats {
    pub gesamt: i64,
    pub aktiv: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct BookingStats {
    pub gesamt: i64,
    pub ausstehend: i64,
    pub bestaetigt: i64,
    pub diesen_monat: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct RevenueStats {
    pub bezahlt: f64,
    pub ausstehend: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct BusStats {
    pub gesamt: i64,
    pub verfuegbar: i64,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct RecentBooking {
    pub buchungsnummer: String,
    pub kunde_name: String,
    pub reise_titel: String,
    pub gesamtpreis: f64,
    pub status: String,
    pub buchungsdatum: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UpcomingTrip {
    pub id: String,
    pub titel: String,
    pub ziel: String,
    pub abfahrt_datum: chrono::NaiveDate,
    pub max_teilnehmer: i32,
    pub aktuelle_buchungen: i32,
    pub verfuegbar: i32,
}
