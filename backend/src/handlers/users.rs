use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{AdminUserRequest, Claims, User, UserResponse};
use crate::services::AuthService;

/// GET /api/admin/users - Alle Benutzer auflisten
pub async fn list_users(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    query: web::Query<UserSearchParams>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    let page = query.seite.unwrap_or(1).max(1);
    let per_page = query.pro_seite.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM users WHERE 1=1");
    let mut count_sql = String::from("SELECT COUNT(*) FROM users WHERE 1=1");

    if let Some(ref rolle) = query.rolle {
        let filter = format!(" AND rolle = '{}'", rolle.replace("'", "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref suche) = query.suche {
        let search = suche.replace("'", "''");
        let filter = format!(
            " AND (email LIKE '%{}%' OR vorname LIKE '%{}%' OR nachname LIKE '%{}%')",
            search, search, search
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(aktiv) = query.aktiv {
        let filter = format!(" AND aktiv = {}", if aktiv { "TRUE" } else { "FALSE" });
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    sql.push_str(&format!(" ORDER BY erstellt_am DESC LIMIT {} OFFSET {}", per_page, offset));

    let users: Vec<User> = sqlx::query_as(&sql)
        .fetch_all(pool.get_ref())
        .await?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(pool.get_ref())
        .await?;

    let users_response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(HttpResponse::Ok().json(UserListResponse {
        benutzer: users_response,
        gesamt: total.0,
        seite: page,
        pro_seite: per_page,
        seiten_gesamt: ((total.0 as f64) / (per_page as f64)).ceil() as u32,
    }))
}

/// GET /api/admin/users/{id} - Einzelnen Benutzer abrufen
pub async fn get_user(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let user_id = path.into_inner();

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Benutzer nicht gefunden".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

/// POST /api/admin/users - Neuen Benutzer erstellen
pub async fn create_user(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<AdminUserRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Prüfen ob E-Mail bereits existiert
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateEntry(
            "Diese E-Mail-Adresse ist bereits registriert".to_string(),
        ));
    }

    let password = body.password.as_ref()
        .ok_or(AppError::ValidationError("Passwort ist erforderlich".to_string()))?;
    
    AuthService::validate_password_strength(password)?;
    let password_hash = AuthService::hash_password(password)?;

    let user_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO users (
            id, email, password_hash, vorname, nachname, telefon,
            strasse, plz, ort, land, geburtsdatum, rolle, email_verifiziert, aktiv
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, TRUE, ?)
        "#
    )
    .bind(&user_id)
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.vorname)
    .bind(&body.nachname)
    .bind(&body.telefon)
    .bind(&body.strasse)
    .bind(&body.plz)
    .bind(&body.ort)
    .bind(&body.land)
    .bind(&body.geburtsdatum)
    .bind(&body.rolle)
    .bind(&body.aktiv)
    .execute(pool.get_ref())
    .await?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

/// PUT /api/admin/users/{id} - Benutzer aktualisieren
pub async fn update_user(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<AdminUserRequest>,
) -> AppResult<HttpResponse> {
    verify_admin(&req, &config)?;
    let user_id = path.into_inner();

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Prüfen ob Benutzer existiert
    let existing: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Benutzer nicht gefunden".to_string()))?;

    // E-Mail-Änderung prüfen
    if body.email != existing.email {
        let email_exists: Option<User> = sqlx::query_as(
            "SELECT * FROM users WHERE email = ? AND id != ?"
        )
        .bind(&body.email)
        .bind(&user_id)
        .fetch_optional(pool.get_ref())
        .await?;

        if email_exists.is_some() {
            return Err(AppError::DuplicateEntry(
                "Diese E-Mail-Adresse ist bereits registriert".to_string(),
            ));
        }
    }

    // Password-Update wenn angegeben
    if let Some(ref password) = body.password {
        AuthService::validate_password_strength(password)?;
        let password_hash = AuthService::hash_password(password)?;
        
        sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
            .bind(&password_hash)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await?;
    }

    // Andere Felder aktualisieren
    sqlx::query(
        r#"
        UPDATE users SET
            email = ?, vorname = ?, nachname = ?, telefon = ?,
            strasse = ?, plz = ?, ort = ?, land = ?, geburtsdatum = ?,
            rolle = ?, aktiv = ?, aktualisiert_am = NOW()
        WHERE id = ?
        "#
    )
    .bind(&body.email)
    .bind(&body.vorname)
    .bind(&body.nachname)
    .bind(&body.telefon)
    .bind(&body.strasse)
    .bind(&body.plz)
    .bind(&body.ort)
    .bind(&body.land)
    .bind(&body.geburtsdatum)
    .bind(&body.rolle)
    .bind(&body.aktiv)
    .bind(&user_id)
    .execute(pool.get_ref())
    .await?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

/// DELETE /api/admin/users/{id} - Benutzer löschen
pub async fn delete_user(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let claims = verify_admin(&req, &config)?;
    let user_id = path.into_inner();

    // Verhindern dass Admin sich selbst löscht
    if claims.sub == user_id {
        return Err(AppError::BadRequest(
            "Sie können sich nicht selbst löschen".to_string(),
        ));
    }

    // Prüfen ob Buchungen existieren
    let booking_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bookings WHERE kunde_id = ?"
    )
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await?;

    if booking_count.0 > 0 {
        // Benutzer nur deaktivieren statt löschen
        sqlx::query("UPDATE users SET aktiv = FALSE, aktualisiert_am = NOW() WHERE id = ?")
            .bind(&user_id)
            .execute(pool.get_ref())
            .await?;

        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Benutzer wurde deaktiviert (Buchungen vorhanden)"
        })));
    }

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&user_id)
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

/// Suchparameter für Benutzer
#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct UserSearchParams {
    pub rolle: Option<String>,
    pub suche: Option<String>,
    pub aktiv: Option<bool>,
    pub seite: Option<u32>,
    pub pro_seite: Option<u32>,
}

/// Paginierte Benutzerliste
#[derive(Debug, Clone, serde::Serialize)]
pub struct UserListResponse {
    pub benutzer: Vec<UserResponse>,
    pub gesamt: i64,
    pub seite: u32,
    pub pro_seite: u32,
    pub seiten_gesamt: u32,
}
