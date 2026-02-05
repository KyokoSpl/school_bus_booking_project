use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Benutzerrolle im System
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "kunde")]
    Kunde,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Kunde
    }
}

/// Benutzer-Entität aus der Datenbank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub vorname: String,
    pub nachname: String,
    pub telefon: Option<String>,
    pub strasse: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub land: Option<String>,
    pub geburtsdatum: Option<NaiveDate>,
    pub rolle: String,
    pub email_verifiziert: bool,
    pub aktiv: bool,
    pub verification_token: Option<String>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<DateTime<Utc>>,
    pub erstellt_am: DateTime<Utc>,
    pub aktualisiert_am: DateTime<Utc>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.rolle == "admin"
    }
}

/// Öffentliche Benutzerantwort (ohne sensible Daten)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub vorname: String,
    pub nachname: String,
    pub telefon: Option<String>,
    pub strasse: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub land: Option<String>,
    pub geburtsdatum: Option<NaiveDate>,
    pub rolle: String,
    pub email_verifiziert: bool,
    pub aktiv: bool,
    pub erstellt_am: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            vorname: user.vorname,
            nachname: user.nachname,
            telefon: user.telefon,
            strasse: user.strasse,
            plz: user.plz,
            ort: user.ort,
            land: user.land,
            geburtsdatum: user.geburtsdatum,
            rolle: user.rolle,
            email_verifiziert: user.email_verifiziert,
            aktiv: user.aktiv,
            erstellt_am: user.erstellt_am,
        }
    }
}

/// Registrierungs-Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Ungültige E-Mail-Adresse"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Passwort muss mindestens 8 Zeichen lang sein"))]
    pub password: String,
    
    #[validate(length(min = 1, message = "Vorname ist erforderlich"))]
    pub vorname: String,
    
    #[validate(length(min = 1, message = "Nachname ist erforderlich"))]
    pub nachname: String,
    
    pub telefon: Option<String>,
    
    pub agb_akzeptiert: bool,
    pub datenschutz_akzeptiert: bool,
}

/// Login-Request
#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login-Response mit JWT Token
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

/// Profil-Update Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "Vorname ist erforderlich"))]
    pub vorname: String,
    
    #[validate(length(min = 1, message = "Nachname ist erforderlich"))]
    pub nachname: String,
    
    pub telefon: Option<String>,
    pub strasse: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub land: Option<String>,
    pub geburtsdatum: Option<NaiveDate>,
}

/// Passwort-Änderung Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    
    #[validate(length(min = 8, message = "Neues Passwort muss mindestens 8 Zeichen lang sein"))]
    pub new_password: String,
}

/// Passwort-Reset Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[validate(email(message = "Ungültige E-Mail-Adresse"))]
    pub email: String,
}

/// Passwort-Reset Bestätigung
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    
    #[validate(length(min = 8, message = "Neues Passwort muss mindestens 8 Zeichen lang sein"))]
    pub new_password: String,
}

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub rolle: String,
    pub exp: i64,     // Expiration timestamp
    pub iat: i64,     // Issued at timestamp
}

/// Admin: Benutzer erstellen/aktualisieren
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AdminUserRequest {
    #[validate(email(message = "Ungültige E-Mail-Adresse"))]
    pub email: String,
    
    pub password: Option<String>,
    
    #[validate(length(min = 1, message = "Vorname ist erforderlich"))]
    pub vorname: String,
    
    #[validate(length(min = 1, message = "Nachname ist erforderlich"))]
    pub nachname: String,
    
    pub telefon: Option<String>,
    pub strasse: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub land: Option<String>,
    pub geburtsdatum: Option<NaiveDate>,
    pub rolle: String,
    pub aktiv: bool,
}
