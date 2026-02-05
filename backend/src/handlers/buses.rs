use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{Bus, BusListResponse, BusResponse, Claims, CreateBusRequest};
use crate::services::AuthService;

/// GET /api/admin/buses - Alle Busse auflisten
pub async fn list_buses(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let buses: Vec<Bus> = sqlx::query_as("SELECT * FROM buses ORDER BY bezeichnung ASC")
        .fetch_all(pool.get_ref())
        .await?;

    let total = buses.len() as i64;
    let buses_response: Vec<BusResponse> = buses.into_iter().map(BusResponse::from).collect();

    Ok(HttpResponse::Ok().json(BusListResponse {
        busse: buses_response,
        gesamt: total,
    }))
}

/// GET /api/admin/buses/{id} - Einzelnen Bus abrufen
pub async fn get_bus(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let bus_id = path.into_inner();

    let bus: Bus = sqlx::query_as("SELECT * FROM buses WHERE id = ?")
        .bind(&bus_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Bus nicht gefunden".to_string()))?;

    Ok(HttpResponse::Ok().json(BusResponse::from(bus)))
}

/// POST /api/admin/buses - Neuen Bus erstellen
pub async fn create_bus(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<CreateBusRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Prüfen ob Kennzeichen bereits existiert
    let existing: Option<Bus> = sqlx::query_as("SELECT * FROM buses WHERE kennzeichen = ?")
        .bind(&body.kennzeichen)
        .fetch_optional(pool.get_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateEntry(
            "Dieses Kennzeichen ist bereits registriert".to_string(),
        ));
    }

    let bus_id = Uuid::new_v4().to_string();
    let status = body.status.clone().unwrap_or_else(|| "verfuegbar".to_string());
    let ausstattung = body.ausstattung.as_ref().map(|a| serde_json::to_string(a).unwrap_or_default());

    sqlx::query(
        r#"
        INSERT INTO buses (id, kennzeichen, bezeichnung, sitzplaetze, ausstattung, baujahr, status, notizen)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&bus_id)
    .bind(&body.kennzeichen)
    .bind(&body.bezeichnung)
    .bind(&body.sitzplaetze)
    .bind(&ausstattung)
    .bind(&body.baujahr)
    .bind(&status)
    .bind(&body.notizen)
    .execute(pool.get_ref())
    .await?;

    let bus: Bus = sqlx::query_as("SELECT * FROM buses WHERE id = ?")
        .bind(&bus_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(BusResponse::from(bus)))
}

/// PUT /api/admin/buses/{id} - Bus aktualisieren
pub async fn update_bus(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<CreateBusRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let bus_id = path.into_inner();

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Prüfen ob Bus existiert
    let existing: Bus = sqlx::query_as("SELECT * FROM buses WHERE id = ?")
        .bind(&bus_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Bus nicht gefunden".to_string()))?;

    // Kennzeichen-Änderung prüfen
    if body.kennzeichen != existing.kennzeichen {
        let kennzeichen_exists: Option<Bus> = sqlx::query_as(
            "SELECT * FROM buses WHERE kennzeichen = ? AND id != ?"
        )
        .bind(&body.kennzeichen)
        .bind(&bus_id)
        .fetch_optional(pool.get_ref())
        .await?;

        if kennzeichen_exists.is_some() {
            return Err(AppError::DuplicateEntry(
                "Dieses Kennzeichen ist bereits registriert".to_string(),
            ));
        }
    }

    let status = body.status.clone().unwrap_or_else(|| "verfuegbar".to_string());
    let ausstattung = body.ausstattung.as_ref().map(|a| serde_json::to_string(a).unwrap_or_default());

    sqlx::query(
        r#"
        UPDATE buses SET
            kennzeichen = ?, bezeichnung = ?, sitzplaetze = ?,
            ausstattung = ?, baujahr = ?, status = ?, notizen = ?,
            aktualisiert_am = NOW()
        WHERE id = ?
        "#
    )
    .bind(&body.kennzeichen)
    .bind(&body.bezeichnung)
    .bind(&body.sitzplaetze)
    .bind(&ausstattung)
    .bind(&body.baujahr)
    .bind(&status)
    .bind(&body.notizen)
    .bind(&bus_id)
    .execute(pool.get_ref())
    .await?;

    let bus: Bus = sqlx::query_as("SELECT * FROM buses WHERE id = ?")
        .bind(&bus_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(BusResponse::from(bus)))
}

/// DELETE /api/admin/buses/{id} - Bus löschen
pub async fn delete_bus(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let bus_id = path.into_inner();

    // Prüfen ob Reisen mit diesem Bus existieren
    let trip_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM trips WHERE bus_id = ? AND status != 'abgeschlossen'"
    )
    .bind(&bus_id)
    .fetch_one(pool.get_ref())
    .await?;

    if trip_count.0 > 0 {
        return Err(AppError::BadRequest(
            "Bus kann nicht gelöscht werden, da aktive Reisen damit verknüpft sind".to_string(),
        ));
    }

    sqlx::query("DELETE FROM buses WHERE id = ?")
        .bind(&bus_id)
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
