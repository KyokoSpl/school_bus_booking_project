use actix_web::{HttpResponse, ResponseError};
use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    // Authentifizierung
    InvalidCredentials,
    Unauthorized,
    TokenExpired,
    TokenInvalid,
    
    // Validation
    ValidationError(String),
    
    // Datenbank
    DatabaseError(String),
    NotFound(String),
    DuplicateEntry(String),
    
    // E-Mail
    EmailError(String),
    
    // Zahlung
    PaymentError(String),
    
    // Allgemein
    InternalError(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidCredentials => write!(f, "Ungültige Anmeldedaten"),
            AppError::Unauthorized => write!(f, "Nicht autorisiert"),
            AppError::TokenExpired => write!(f, "Token abgelaufen"),
            AppError::TokenInvalid => write!(f, "Ungültiger Token"),
            AppError::ValidationError(msg) => write!(f, "Validierungsfehler: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Datenbankfehler: {}", msg),
            AppError::NotFound(msg) => write!(f, "Nicht gefunden: {}", msg),
            AppError::DuplicateEntry(msg) => write!(f, "Duplikat: {}", msg),
            AppError::EmailError(msg) => write!(f, "E-Mail-Fehler: {}", msg),
            AppError::PaymentError(msg) => write!(f, "Zahlungsfehler: {}", msg),
            AppError::InternalError(msg) => write!(f, "Interner Fehler: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Ungültige Anfrage: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        #[derive(serde::Serialize)]
        struct ErrorResponse {
            error: String,
            message: String,
        }

        let (status, error_type) = match self {
            AppError::InvalidCredentials => (actix_web::http::StatusCode::UNAUTHORIZED, "invalid_credentials"),
            AppError::Unauthorized => (actix_web::http::StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::TokenExpired => (actix_web::http::StatusCode::UNAUTHORIZED, "token_expired"),
            AppError::TokenInvalid => (actix_web::http::StatusCode::UNAUTHORIZED, "token_invalid"),
            AppError::ValidationError(_) => (actix_web::http::StatusCode::BAD_REQUEST, "validation_error"),
            AppError::DatabaseError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AppError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "not_found"),
            AppError::DuplicateEntry(_) => (actix_web::http::StatusCode::CONFLICT, "duplicate_entry"),
            AppError::EmailError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "email_error"),
            AppError::PaymentError(_) => (actix_web::http::StatusCode::BAD_REQUEST, "payment_error"),
            AppError::InternalError(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
            AppError::BadRequest(_) => (actix_web::http::StatusCode::BAD_REQUEST, "bad_request"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        })
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Datensatz nicht gefunden".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.code().map(|c| c == "23000").unwrap_or(false) {
                    AppError::DuplicateEntry("Eintrag existiert bereits".to_string())
                } else {
                    AppError::DatabaseError(db_err.to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::TokenInvalid,
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AppError::InvalidCredentials
    }
}

impl From<lettre::error::Error> for AppError {
    fn from(err: lettre::error::Error) -> Self {
        AppError::EmailError(err.to_string())
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        AppError::EmailError(err.to_string())
    }
}
