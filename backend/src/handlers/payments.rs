use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{
    Booking, Claims, InitiatePaymentRequest, Payment, PaymentConfirmation, PaymentResponse,
};
use crate::services::{AuthService, PaymentService};

/// POST /api/payments/initiate - Zahlung initiieren
pub async fn initiate_payment(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    body: web::Json<InitiatePaymentRequest>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;

    // Buchung laden
    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&body.buchung_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Buchung nicht gefunden".to_string()))?;

    // Prüfen ob Buchung dem Benutzer gehört
    if booking.kunde_id != claims.sub && claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }

    // Prüfen ob bereits bezahlt
    if booking.zahlungsstatus == "bezahlt" {
        return Err(AppError::BadRequest("Buchung ist bereits bezahlt".to_string()));
    }

    // Zahlungsdetails für den Mock-Service vorbereiten
    let payment_details = match body.methode.to_lowercase().as_str() {
        "kreditkarte" | "creditcard" => {
            let card = body.kreditkarte.as_ref()
                .ok_or(AppError::ValidationError("Kreditkartendaten fehlen".to_string()))?;
            serde_json::json!({
                "kartennummer": card.kartennummer,
                "inhaber": card.inhaber,
                "ablaufdatum": card.ablaufdatum,
                "cvv": card.cvv
            })
        }
        "paypal" => {
            serde_json::json!({
                "email": claims.email
            })
        }
        "rechnung" => {
            serde_json::json!({
                "kunde": format!("{}", claims.email)
            })
        }
        _ => {
            return Err(AppError::ValidationError(format!(
                "Ungültige Zahlungsmethode: {}",
                body.methode
            )));
        }
    };

    // Zahlung verarbeiten (Mock)
    let mock_result = PaymentService::process_payment(
        &body.methode,
        booking.gesamtpreis,
        &payment_details,
    ).await?;

    // Zahlung in DB speichern
    let payment_id = Uuid::new_v4().to_string();
    let status = if mock_result.success { "erfolgreich" } else { "fehlgeschlagen" };

    sqlx::query(
        r#"
        INSERT INTO payments (id, buchung_id, betrag, methode, status, transaktions_id, zahlungsdetails)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&payment_id)
    .bind(&body.buchung_id)
    .bind(booking.gesamtpreis)
    .bind(&body.methode)
    .bind(status)
    .bind(&mock_result.transaction_id)
    .bind(serde_json::to_string(&payment_details).ok())
    .execute(pool.get_ref())
    .await?;

    // Buchungsstatus aktualisieren wenn erfolgreich
    if mock_result.success {
        let new_booking_status = if body.methode.to_lowercase() == "rechnung" {
            "bestaetigt"  // Rechnung: Bestätigt aber noch nicht bezahlt
        } else {
            "bezahlt"  // Andere: Direkt bezahlt
        };

        let new_payment_status = if body.methode.to_lowercase() == "rechnung" {
            "ausstehend"  // Rechnung: Zahlung noch ausstehend
        } else {
            "bezahlt"
        };

        sqlx::query(
            "UPDATE bookings SET status = ?, zahlungsstatus = ?, aktualisiert_am = NOW() WHERE id = ?"
        )
        .bind(new_booking_status)
        .bind(new_payment_status)
        .bind(&body.buchung_id)
        .execute(pool.get_ref())
        .await?;
    }

    let payment: Payment = sqlx::query_as("SELECT * FROM payments WHERE id = ?")
        .bind(&payment_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(PaymentConfirmation {
        erfolg: mock_result.success,
        transaktions_id: mock_result.transaction_id,
        nachricht: mock_result.message,
        zahlung: PaymentResponse::from(payment),
    }))
}

/// GET /api/payments/{id} - Zahlung abrufen
pub async fn get_payment(
    req: HttpRequest,
    pool: web::Data<MySqlPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let claims = extract_claims(&req, &config)?;
    let payment_id = path.into_inner();

    let payment: Payment = sqlx::query_as("SELECT * FROM payments WHERE id = ?")
        .bind(&payment_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or(AppError::NotFound("Zahlung nicht gefunden".to_string()))?;

    // Prüfen ob Zahlung dem Benutzer gehört (über Buchung)
    let booking: Booking = sqlx::query_as("SELECT * FROM bookings WHERE id = ?")
        .bind(&payment.buchung_id)
        .fetch_one(pool.get_ref())
        .await?;

    if booking.kunde_id != claims.sub && claims.rolle != "admin" {
        return Err(AppError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(PaymentResponse::from(payment)))
}

/// Hilfsfunktion: Claims aus Header extrahieren
fn extract_claims(req: &HttpRequest, config: &Config) -> AppResult<Claims> {
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
