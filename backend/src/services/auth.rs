use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{Claims, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct AuthService;

impl AuthService {
    /// Hasht ein Passwort mit Argon2
    pub fn hash_password(password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalError("Passwort konnte nicht gehasht werden".to_string()))?
            .to_string();
        
        Ok(password_hash)
    }

    /// Verifiziert ein Passwort gegen einen Hash
    pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::InternalError("Ungültiger Passwort-Hash".to_string()))?;
        
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generiert ein JWT Token für einen Benutzer
    pub fn generate_token(user: &User, config: &Config) -> AppResult<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(config.jwt_expiration_hours);

        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            rolle: user.rolle.clone(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validiert ein JWT Token und gibt die Claims zurück
    pub fn validate_token(token: &str, config: &Config) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Generiert einen zufälligen Token (für E-Mail-Verifikation, Passwort-Reset)
    pub fn generate_random_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let token: String = (0..64)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'a' + idx - 10) as char
                }
            })
            .collect();
        token
    }

    /// Validiert Passwort-Stärke
    pub fn validate_password_strength(password: &str) -> AppResult<()> {
        if password.len() < 8 {
            return Err(AppError::ValidationError(
                "Passwort muss mindestens 8 Zeichen lang sein".to_string(),
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(AppError::ValidationError(
                "Passwort muss Groß- und Kleinbuchstaben sowie Zahlen enthalten".to_string(),
            ));
        }

        Ok(())
    }
}
