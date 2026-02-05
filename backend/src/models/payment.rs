use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Zahlungs-Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Ausstehend,
    Verarbeitet,
    Erfolgreich,
    Fehlgeschlagen,
    Erstattet,
}

impl Default for PaymentStatus {
    fn default() -> Self {
        PaymentStatus::Ausstehend
    }
}

/// Zahlungsmethode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Kreditkarte,
    PayPal,
    Rechnung,
}

/// Zahlungs-Entität aus der Datenbank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: String,
    pub buchung_id: String,
    pub betrag: f64,
    pub methode: String,
    pub status: String,
    pub transaktions_id: Option<String>,
    pub zahlungsdetails: Option<String>,  // JSON als String
    pub erstellt_am: DateTime<Utc>,
    pub aktualisiert_am: DateTime<Utc>,
}

/// Zahlungs-Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: String,
    pub buchung_id: String,
    pub betrag: f64,
    pub methode: String,
    pub status: String,
    pub transaktions_id: Option<String>,
    pub erstellt_am: DateTime<Utc>,
}

impl From<Payment> for PaymentResponse {
    fn from(payment: Payment) -> Self {
        PaymentResponse {
            id: payment.id,
            buchung_id: payment.buchung_id,
            betrag: payment.betrag,
            methode: payment.methode,
            status: payment.status,
            transaktions_id: payment.transaktions_id,
            erstellt_am: payment.erstellt_am,
        }
    }
}

/// Zahlung initiieren Request
#[derive(Debug, Clone, Deserialize)]
pub struct InitiatePaymentRequest {
    pub buchung_id: String,
    pub methode: String,  // "kreditkarte", "paypal", "rechnung"
    pub kreditkarte: Option<CreditCardInfo>,
}

/// Kreditkarteninfo (für Mock-Zahlung)
#[derive(Debug, Clone, Deserialize)]
pub struct CreditCardInfo {
    pub kartennummer: String,  // Nur letzte 4 Ziffern speichern
    pub inhaber: String,
    pub ablaufdatum: String,  // MM/YY
    pub cvv: String,
}

/// Zahlung bestätigen Response
#[derive(Debug, Clone, Serialize)]
pub struct PaymentConfirmation {
    pub erfolg: bool,
    pub transaktions_id: String,
    pub nachricht: String,
    pub zahlung: PaymentResponse,
}

/// Mock-Zahlungs-Response (simuliert externe Zahlungsanbieter)
#[derive(Debug, Clone, Serialize)]
pub struct MockPaymentResponse {
    pub success: bool,
    pub transaction_id: String,
    pub message: String,
    pub provider: String,
}
