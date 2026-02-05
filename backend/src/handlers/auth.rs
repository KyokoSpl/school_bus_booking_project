use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::MySqlPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{
    AuthResponse, ChangePasswordRequest, Claims, LoginRequest, PasswordResetConfirmRequest,
    PasswordResetRequest, RegisterRequest, UpdateProfileRequest, User, UserResponse,
};
use crate::services::{AuthService, EmailService};

/// POST /api/auth/register - Benutzer registrieren
pub async fn register(
    pool: web::Data<MySqlPool>,
    email_service: web::Data<EmailService>,
    body: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    // Validierung
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    if !body.agb_akzeptiert || !body.datenschutz_akzeptiert {
        return Err(AppError::ValidationError(
            "AGB und Datenschutzerklärung müssen akzeptiert werden".to_string(),
        ));
    }

    // Passwort-Stärke prüfen
    AuthService::validate_password_strength(&body.password)?;

    // Prüfen ob E-Mail bereits existiert
    let existing: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await?;

    if existing.is_some() {
        return Err(AppError::DuplicateEntry(
            "Diese E-Mail-Adresse ist bereits registriert".to_string(),
        ));
    }

    // Passwort hashen
    let password_hash = AuthService::hash_password(&body.password)?;
    let verification_token = AuthService::generate_random_token();
    let user_id = Uuid::new_v4().to_string();

    // Benutzer erstellen
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, vorname, nachname, telefon, verification_token, rolle, email_verifiziert, aktiv)
        VALUES (?, ?, ?, ?, ?, ?, ?, 'kunde', FALSE, TRUE)
        "#
    )
    .bind(&user_id)
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.vorname)
    .bind(&body.nachname)
    .bind(&body.telefon)
    .bind(&verification_token)
    .execute(pool.get_ref())
    .await?;

    // Benutzer laden für Response
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await?;

    // Verifizierungs-E-Mail senden (async, nicht blockierend)
    let email_service = email_service.clone();
    let user_clone = user.clone();
    let token_clone = verification_token.clone();
    tokio::spawn(async move {
        if let Err(e) = email_service.send_verification_email(&user_clone, &token_clone).await {
            tracing::error!("Fehler beim Senden der Verifizierungs-E-Mail: {}", e);
        }
    });

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Registrierung erfolgreich. Bitte bestätigen Sie Ihre E-Mail-Adresse.",
        "user": UserResponse::from(user)
    })))
}

/// POST /api/auth/login - Benutzer anmelden
pub async fn login(
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    // Benutzer suchen
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Prüfen ob Konto aktiv ist
    if !user.aktiv {
        return Err(AppError::BadRequest("Konto ist deaktiviert".to_string()));
    }

    // Passwort verifizieren
    if !AuthService::verify_password(&body.password, &user.password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    // Token generieren
    let token = AuthService::generate_token(&user, &config)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: config.jwt_expiration_hours * 3600,
        user: UserResponse::from(user),
    }))
}

/// GET /api/auth/verify-email - E-Mail verifizieren
pub async fn verify_email(
    pool: web::Data<MySqlPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> AppResult<HttpResponse> {
    let token = query
        .get("token")
        .ok_or(AppError::BadRequest("Token fehlt".to_string()))?;

    // Benutzer mit diesem Token finden
    let user: User = sqlx::query_as(
        "SELECT * FROM users WHERE verification_token = ? AND email_verifiziert = FALSE"
    )
    .bind(token)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AppError::BadRequest("Ungültiger oder abgelaufener Token".to_string()))?;

    // E-Mail verifizieren
    sqlx::query(
        "UPDATE users SET email_verifiziert = TRUE, verification_token = NULL, aktualisiert_am = NOW() WHERE id = ?"
    )
    .bind(&user.id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "E-Mail-Adresse erfolgreich bestätigt"
    })))
}

/// POST /api/auth/forgot-password - Passwort vergessen
pub async fn forgot_password(
    pool: web::Data<MySqlPool>,
    email_service: web::Data<EmailService>,
    body: web::Json<PasswordResetRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Benutzer suchen
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await?;

    // Aus Sicherheitsgründen immer die gleiche Antwort geben
    if let Some(user) = user {
        let reset_token = AuthService::generate_random_token();
        let expires = Utc::now() + Duration::hours(1);

        // Reset-Token speichern
        sqlx::query(
            "UPDATE users SET password_reset_token = ?, password_reset_expires = ? WHERE id = ?"
        )
        .bind(&reset_token)
        .bind(expires)
        .bind(&user.id)
        .execute(pool.get_ref())
        .await?;

        // E-Mail senden
        let email_service = email_service.clone();
        tokio::spawn(async move {
            if let Err(e) = email_service.send_password_reset_email(&user, &reset_token).await {
                tracing::error!("Fehler beim Senden der Reset-E-Mail: {}", e);
            }
        });
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Falls ein Konto mit dieser E-Mail existiert, wurde eine E-Mail zum Zurücksetzen des Passworts gesendet."
    })))
}

/// POST /api/auth/reset-password - Passwort zurücksetzen
pub async fn reset_password(
    pool: web::Data<MySqlPool>,
    body: web::Json<PasswordResetConfirmRequest>,
) -> AppResult<HttpResponse> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    AuthService::validate_password_strength(&body.new_password)?;

    // Benutzer mit gültigem Reset-Token finden
    let user: User = sqlx::query_as(
        "SELECT * FROM users WHERE password_reset_token = ? AND password_reset_expires > NOW()"
    )
    .bind(&body.token)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AppError::BadRequest("Ungültiger oder abgelaufener Token".to_string()))?;

    // Neues Passwort hashen und speichern
    let password_hash = AuthService::hash_password(&body.new_password)?;

    sqlx::query(
        "UPDATE users SET password_hash = ?, password_reset_token = NULL, password_reset_expires = NULL, aktualisiert_am = NOW() WHERE id = ?"
    )
    .bind(&password_hash)
    .bind(&user.id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Passwort erfolgreich zurückgesetzt"
    })))
}

/// GET /api/user/profile - Eigenes Profil abrufen
pub async fn get_profile(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims_from_header(&req, &config)?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

/// PUT /api/user/profile - Eigenes Profil aktualisieren
pub async fn update_profile(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<UpdateProfileRequest>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims_from_header(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    sqlx::query(
        r#"
        UPDATE users SET
            vorname = ?,
            nachname = ?,
            telefon = ?,
            strasse = ?,
            plz = ?,
            ort = ?,
            land = ?,
            geburtsdatum = ?,
            aktualisiert_am = NOW()
        WHERE id = ?
        "#
    )
    .bind(&body.vorname)
    .bind(&body.nachname)
    .bind(&body.telefon)
    .bind(&body.strasse)
    .bind(&body.plz)
    .bind(&body.ort)
    .bind(&body.land)
    .bind(&body.geburtsdatum)
    .bind(&claims.sub)
    .execute(pool.get_ref())
    .await?;

    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

/// POST /api/user/change-password - Passwort ändern
pub async fn change_password(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<ChangePasswordRequest>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims_from_header(&req, &config)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    AuthService::validate_password_strength(&body.new_password)?;

    // Aktuellen Benutzer laden
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await?;

    // Aktuelles Passwort verifizieren
    if !AuthService::verify_password(&body.current_password, &user.password_hash)? {
        return Err(AppError::BadRequest("Aktuelles Passwort ist falsch".to_string()));
    }

    // Neues Passwort hashen und speichern
    let password_hash = AuthService::hash_password(&body.new_password)?;

    sqlx::query("UPDATE users SET password_hash = ?, aktualisiert_am = NOW() WHERE id = ?")
        .bind(&password_hash)
        .bind(&claims.sub)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Passwort erfolgreich geändert"
    })))
}

/// Hilfsfunktion: Claims aus Authorization Header extrahieren
fn extract_claims_from_header(req: &HttpRequest, config: &Config) -> AppResult<Claims> {
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
