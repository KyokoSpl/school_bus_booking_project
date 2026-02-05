use crate::error::{AppError, AppResult};
use crate::models::{MockPaymentResponse, Payment, PaymentResponse};
use chrono::Utc;
use rand::Rng;
use uuid::Uuid;

/// Zahlungs-Service (Mock-Implementierung)
/// Simuliert externe Zahlungsanbieter wie Stripe oder PayPal
pub struct PaymentService;

impl PaymentService {
    /// Simuliert eine Kreditkartenzahlung
    pub async fn process_credit_card(
        amount: f64,
        card_number: &str,
        card_holder: &str,
        expiry: &str,
        _cvv: &str,
    ) -> AppResult<MockPaymentResponse> {
        // Simuliere Netzwerk-Latenz
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Einfache Validierung (Mock)
        let card_last_four = card_number.chars().rev().take(4).collect::<String>();
        
        // Simuliere Fehler für bestimmte Kartennummern (für Tests)
        if card_number.ends_with("0000") {
            return Err(AppError::PaymentError("Karte abgelehnt".to_string()));
        }

        if card_number.ends_with("9999") {
            return Err(AppError::PaymentError("Unzureichende Deckung".to_string()));
        }

        // Erfolgreiche Zahlung simulieren
        let transaction_id = format!("CC-{}-{}", Utc::now().format("%Y%m%d%H%M%S"), Self::generate_reference());
        
        tracing::info!(
            "Kreditkartenzahlung simuliert: {} EUR, Karte: ****{}, Inhaber: {}",
            amount, card_last_four, card_holder
        );

        Ok(MockPaymentResponse {
            success: true,
            transaction_id,
            message: format!("Zahlung von {:.2} EUR erfolgreich verarbeitet", amount),
            provider: "Stripe (Mock)".to_string(),
        })
    }

    /// Simuliert eine PayPal-Zahlung
    pub async fn process_paypal(amount: f64, email: &str) -> AppResult<MockPaymentResponse> {
        // Simuliere Netzwerk-Latenz
        tokio::time::sleep(tokio::time::Duration::from_millis(700)).await;

        // Simuliere Fehler für bestimmte E-Mails (für Tests)
        if email.contains("fail") {
            return Err(AppError::PaymentError("PayPal-Zahlung fehlgeschlagen".to_string()));
        }

        let transaction_id = format!("PP-{}-{}", Utc::now().format("%Y%m%d%H%M%S"), Self::generate_reference());
        
        tracing::info!("PayPal-Zahlung simuliert: {} EUR, E-Mail: {}", amount, email);

        Ok(MockPaymentResponse {
            success: true,
            transaction_id,
            message: format!("PayPal-Zahlung von {:.2} EUR erfolgreich", amount),
            provider: "PayPal (Mock)".to_string(),
        })
    }

    /// Erstellt eine Rechnungs-Zahlung (immer erfolgreich, da auf Rechnung)
    pub async fn create_invoice(amount: f64, customer_name: &str) -> AppResult<MockPaymentResponse> {
        // Simuliere Netzwerk-Latenz
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        let invoice_number = format!("RE-{}-{}", Utc::now().format("%Y%m%d"), Self::generate_reference());
        
        tracing::info!("Rechnung erstellt: {} EUR für {}", amount, customer_name);

        Ok(MockPaymentResponse {
            success: true,
            transaction_id: invoice_number,
            message: format!("Rechnung über {:.2} EUR erstellt. Zahlungsziel: 14 Tage", amount),
            provider: "Rechnung".to_string(),
        })
    }

    /// Simuliert eine Erstattung
    pub async fn process_refund(
        original_transaction_id: &str,
        amount: f64,
    ) -> AppResult<MockPaymentResponse> {
        tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

        let refund_id = format!("RF-{}-{}", Utc::now().format("%Y%m%d%H%M%S"), Self::generate_reference());
        
        tracing::info!(
            "Erstattung simuliert: {} EUR für Transaktion {}",
            amount, original_transaction_id
        );

        Ok(MockPaymentResponse {
            success: true,
            transaction_id: refund_id,
            message: format!("Erstattung von {:.2} EUR erfolgreich verarbeitet", amount),
            provider: "Refund Service (Mock)".to_string(),
        })
    }

    /// Generiert eine zufällige Referenznummer
    fn generate_reference() -> String {
        let mut rng = rand::thread_rng();
        let reference: String = (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'A' + idx - 10) as char
                }
            })
            .collect();
        reference
    }

    /// Verarbeitet eine Zahlung basierend auf der Zahlungsmethode
    pub async fn process_payment(
        method: &str,
        amount: f64,
        details: &serde_json::Value,
    ) -> AppResult<MockPaymentResponse> {
        match method.to_lowercase().as_str() {
            "kreditkarte" | "creditcard" => {
                let card_number = details["kartennummer"].as_str().unwrap_or("");
                let card_holder = details["inhaber"].as_str().unwrap_or("");
                let expiry = details["ablaufdatum"].as_str().unwrap_or("");
                let cvv = details["cvv"].as_str().unwrap_or("");
                
                Self::process_credit_card(amount, card_number, card_holder, expiry, cvv).await
            }
            "paypal" => {
                let email = details["email"].as_str().unwrap_or("");
                Self::process_paypal(amount, email).await
            }
            "rechnung" | "invoice" => {
                let customer_name = details["kunde"].as_str().unwrap_or("Unbekannt");
                Self::create_invoice(amount, customer_name).await
            }
            _ => Err(AppError::PaymentError(format!(
                "Ungültige Zahlungsmethode: {}",
                method
            ))),
        }
    }
}
