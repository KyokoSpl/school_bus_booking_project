use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::Claims;
use crate::services::AuthService;
use actix_web::{dev::ServiceRequest, web, HttpMessage};

/// Extrahiert und validiert das JWT Token aus dem Authorization Header
pub async fn extract_claims(req: &ServiceRequest, config: &Config) -> AppResult<Claims> {
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

/// Authentifizierungs-Guard für geschützte Routen
pub struct AuthGuard;

impl AuthGuard {
    /// Validiert den Token und fügt die Claims zur Request hinzu
    pub async fn validate(
        req: &ServiceRequest,
        config: &web::Data<Config>,
    ) -> AppResult<Claims> {
        let claims = extract_claims(req, config).await?;
        req.extensions_mut().insert(claims.clone());
        Ok(claims)
    }

    /// Validiert, dass der Benutzer ein Admin ist
    pub async fn validate_admin(
        req: &ServiceRequest,
        config: &web::Data<Config>,
    ) -> AppResult<Claims> {
        let claims = Self::validate(req, config).await?;
        if claims.rolle != "admin" {
            return Err(AppError::Unauthorized);
        }
        Ok(claims)
    }
}

/// Extrahiert Claims aus dem Request (für Handler)
pub fn get_claims_from_request(req: &actix_web::HttpRequest) -> AppResult<Claims> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or(AppError::Unauthorized)
}
