use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Buchungs-Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BookingStatus {
    Ausstehend,
    Bestaetigt,
    Bezahlt,
    Storniert,
}

impl Default for BookingStatus {
    fn default() -> Self {
        BookingStatus::Ausstehend
    }
}

/// Buchungs-Entität aus der Datenbank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: String,
    pub buchungsnummer: String,
    pub kunde_id: String,
    pub reise_id: String,
    pub anzahl_personen: i32,
    pub gesamtpreis: f64,
    pub status: String,
    pub buchungsdatum: DateTime<Utc>,
    pub mitreisende: Option<String>,  // JSON als String
    pub bemerkungen: Option<String>,
    pub zahlungsstatus: String,
    pub zahlungsmethode: Option<String>,
    pub erstellt_am: DateTime<Utc>,
    pub aktualisiert_am: DateTime<Utc>,
}

/// Mitreisender (in JSON gespeichert)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mitreisender {
    pub vorname: String,
    pub nachname: String,
    pub geburtsdatum: Option<String>,
}

/// Buchungs-Response mit Reise- und Kunden-Infos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingResponse {
    pub id: String,
    pub buchungsnummer: String,
    pub kunde_id: String,
    pub kunde_name: Option<String>,
    pub kunde_email: Option<String>,
    pub reise_id: String,
    pub reise_titel: Option<String>,
    pub reise_ziel: Option<String>,
    pub reise_datum: Option<String>,
    pub anzahl_personen: i32,
    pub gesamtpreis: f64,
    pub status: String,
    pub buchungsdatum: DateTime<Utc>,
    pub mitreisende: Vec<Mitreisender>,
    pub bemerkungen: Option<String>,
    pub zahlungsstatus: String,
    pub zahlungsmethode: Option<String>,
    pub erstellt_am: DateTime<Utc>,
}

impl From<Booking> for BookingResponse {
    fn from(booking: Booking) -> Self {
        BookingResponse {
            id: booking.id,
            buchungsnummer: booking.buchungsnummer,
            kunde_id: booking.kunde_id,
            kunde_name: None,
            kunde_email: None,
            reise_id: booking.reise_id,
            reise_titel: None,
            reise_ziel: None,
            reise_datum: None,
            anzahl_personen: booking.anzahl_personen,
            gesamtpreis: booking.gesamtpreis,
            status: booking.status,
            buchungsdatum: booking.buchungsdatum,
            mitreisende: booking.mitreisende
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default(),
            bemerkungen: booking.bemerkungen,
            zahlungsstatus: booking.zahlungsstatus,
            zahlungsmethode: booking.zahlungsmethode,
            erstellt_am: booking.erstellt_am,
        }
    }
}

/// Buchung erstellen Request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateBookingRequest {
    pub reise_id: String,
    
    #[validate(range(min = 1, max = 50, message = "Anzahl Personen muss zwischen 1 und 50 liegen"))]
    pub anzahl_personen: i32,
    
    pub mitreisende: Option<Vec<Mitreisender>>,
    pub bemerkungen: Option<String>,
    pub zahlungsmethode: String,  // "kreditkarte", "paypal", "rechnung"
}

/// Buchung aktualisieren Request (Admin)
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBookingRequest {
    pub status: Option<String>,
    pub zahlungsstatus: Option<String>,
    pub bemerkungen: Option<String>,
}

/// Buchungs-Suchfilter
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BookingSearchParams {
    pub kunde_id: Option<String>,
    pub reise_id: Option<String>,
    pub status: Option<String>,
    pub zahlungsstatus: Option<String>,
    pub von_datum: Option<String>,
    pub bis_datum: Option<String>,
    pub buchungsnummer: Option<String>,
    pub seite: Option<u32>,
    pub pro_seite: Option<u32>,
}

/// Paginierte Buchungs-Liste
#[derive(Debug, Clone, Serialize)]
pub struct BookingListResponse {
    pub buchungen: Vec<BookingResponse>,
    pub gesamt: i64,
    pub seite: u32,
    pub pro_seite: u32,
    pub seiten_gesamt: u32,
}

/// Buchungs-Stornierung Request
#[derive(Debug, Clone, Deserialize)]
pub struct CancelBookingRequest {
    pub grund: Option<String>,
}
