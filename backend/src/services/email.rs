use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::{Booking, Trip, User};
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
    frontend_url: String,
}

impl EmailService {
    pub fn new(config: &Config) -> AppResult<Self> {
        let creds = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .map_err(|e| AppError::EmailError(e.to_string()))?
            .credentials(creds)
            .port(config.smtp_port)
            .build();

        Ok(Self {
            mailer,
            from_email: config.smtp_from_email.clone(),
            from_name: config.smtp_from_name.clone(),
            frontend_url: config.frontend_url.clone(),
        })
    }

    fn build_from_mailbox(&self) -> AppResult<Mailbox> {
        format!("{} <{}>", self.from_name, self.from_email)
            .parse()
            .map_err(|_| AppError::EmailError("Ungültige Absender-Adresse".to_string()))
    }

    /// Sendet eine E-Mail-Verifizierung
    pub async fn send_verification_email(&self, user: &User, token: &str) -> AppResult<()> {
        let verification_url = format!("{}/verify-email?token={}", self.frontend_url, token);
        
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #2563EB; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #2563EB; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ padding: 20px; text-align: center; font-size: 12px; color: #666; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Sonnenschein Reisen</h1>
                    </div>
                    <div class="content">
                        <h2>Willkommen, {} {}!</h2>
                        <p>Vielen Dank für Ihre Registrierung bei Sonnenschein Reisen.</p>
                        <p>Bitte bestätigen Sie Ihre E-Mail-Adresse, indem Sie auf den folgenden Button klicken:</p>
                        <a href="{}" class="button">E-Mail bestätigen</a>
                        <p>Oder kopieren Sie diesen Link in Ihren Browser:</p>
                        <p style="word-break: break-all;">{}</p>
                        <p>Dieser Link ist 24 Stunden gültig.</p>
                    </div>
                    <div class="footer">
                        <p>Sonnenschein Reisen GmbH | Musterstraße 1 | 80331 München</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user.vorname, user.nachname, verification_url, verification_url
        );

        let email = Message::builder()
            .from(self.build_from_mailbox()?)
            .to(format!("{} {} <{}>", user.vorname, user.nachname, user.email)
                .parse()
                .map_err(|_| AppError::EmailError("Ungültige Empfänger-Adresse".to_string()))?)
            .subject("Bitte bestätigen Sie Ihre E-Mail-Adresse")
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(email)
            .await
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        tracing::info!("Verifizierungs-E-Mail gesendet an: {}", user.email);
        Ok(())
    }

    /// Sendet eine Buchungsbestätigung
    pub async fn send_booking_confirmation(
        &self,
        user: &User,
        booking: &Booking,
        trip: &Trip,
    ) -> AppResult<()> {
        let booking_url = format!("{}/buchungen/{}", self.frontend_url, booking.id);
        
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #2563EB; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .booking-details {{ background-color: white; padding: 15px; border-radius: 4px; margin: 15px 0; }}
                    .booking-details h3 {{ margin-top: 0; color: #2563EB; }}
                    .detail-row {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #eee; }}
                    .total {{ font-size: 18px; font-weight: bold; color: #2563EB; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #2563EB; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ padding: 20px; text-align: center; font-size: 12px; color: #666; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Buchungsbestätigung</h1>
                    </div>
                    <div class="content">
                        <h2>Vielen Dank für Ihre Buchung, {} {}!</h2>
                        <p>Wir freuen uns, Sie auf unserer Reise begrüßen zu dürfen.</p>
                        
                        <div class="booking-details">
                            <h3>Ihre Buchungsdetails</h3>
                            <div class="detail-row">
                                <span>Buchungsnummer:</span>
                                <span><strong>{}</strong></span>
                            </div>
                            <div class="detail-row">
                                <span>Reise:</span>
                                <span>{}</span>
                            </div>
                            <div class="detail-row">
                                <span>Ziel:</span>
                                <span>{}</span>
                            </div>
                            <div class="detail-row">
                                <span>Abfahrt:</span>
                                <span>{} um {}</span>
                            </div>
                            <div class="detail-row">
                                <span>Rückkehr:</span>
                                <span>{} um {}</span>
                            </div>
                            <div class="detail-row">
                                <span>Abfahrtsort:</span>
                                <span>{}</span>
                            </div>
                            <div class="detail-row">
                                <span>Anzahl Personen:</span>
                                <span>{}</span>
                            </div>
                            <div class="detail-row total">
                                <span>Gesamtpreis:</span>
                                <span>{:.2} €</span>
                            </div>
                        </div>

                        <a href="{}" class="button">Buchung anzeigen</a>
                        
                        <p><strong>Wichtige Hinweise:</strong></p>
                        <ul>
                            <li>Bitte seien Sie mindestens 15 Minuten vor Abfahrt am Treffpunkt</li>
                            <li>Bringen Sie einen gültigen Personalausweis oder Reisepass mit</li>
                            <li>Bei Fragen erreichen Sie uns unter info@sonnenschein-reisen.de</li>
                        </ul>
                    </div>
                    <div class="footer">
                        <p>Sonnenschein Reisen GmbH | Musterstraße 1 | 80331 München</p>
                        <p>Tel: +49 89 12345678 | E-Mail: info@sonnenschein-reisen.de</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user.vorname, user.nachname,
            booking.buchungsnummer,
            trip.titel,
            trip.ziel,
            trip.abfahrt_datum, trip.abfahrt_zeit,
            trip.rueckkehr_datum, trip.rueckkehr_zeit,
            trip.abfahrtsort,
            booking.anzahl_personen,
            booking.gesamtpreis,
            booking_url
        );

        let email = Message::builder()
            .from(self.build_from_mailbox()?)
            .to(format!("{} {} <{}>", user.vorname, user.nachname, user.email)
                .parse()
                .map_err(|_| AppError::EmailError("Ungültige Empfänger-Adresse".to_string()))?)
            .subject(format!("Buchungsbestätigung #{} - {}", booking.buchungsnummer, trip.titel))
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(email)
            .await
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        tracing::info!("Buchungsbestätigung gesendet an: {}", user.email);
        Ok(())
    }

    /// Sendet einen Passwort-Reset-Link
    pub async fn send_password_reset_email(&self, user: &User, token: &str) -> AppResult<()> {
        let reset_url = format!("{}/reset-password?token={}", self.frontend_url, token);
        
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #2563EB; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #2563EB; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .warning {{ background-color: #FEF3CD; border: 1px solid #FFEEBA; padding: 15px; border-radius: 4px; margin: 15px 0; }}
                    .footer {{ padding: 20px; text-align: center; font-size: 12px; color: #666; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Passwort zurücksetzen</h1>
                    </div>
                    <div class="content">
                        <h2>Hallo {} {}!</h2>
                        <p>Sie haben eine Anfrage zum Zurücksetzen Ihres Passworts gestellt.</p>
                        <p>Klicken Sie auf den folgenden Button, um ein neues Passwort zu setzen:</p>
                        <a href="{}" class="button">Passwort zurücksetzen</a>
                        <p>Oder kopieren Sie diesen Link in Ihren Browser:</p>
                        <p style="word-break: break-all;">{}</p>
                        
                        <div class="warning">
                            <strong>Hinweis:</strong> Dieser Link ist nur 1 Stunde gültig. Falls Sie diese Anfrage nicht gestellt haben, können Sie diese E-Mail ignorieren.
                        </div>
                    </div>
                    <div class="footer">
                        <p>Sonnenschein Reisen GmbH | Musterstraße 1 | 80331 München</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user.vorname, user.nachname, reset_url, reset_url
        );

        let email = Message::builder()
            .from(self.build_from_mailbox()?)
            .to(format!("{} {} <{}>", user.vorname, user.nachname, user.email)
                .parse()
                .map_err(|_| AppError::EmailError("Ungültige Empfänger-Adresse".to_string()))?)
            .subject("Passwort zurücksetzen")
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        self.mailer
            .send(email)
            .await
            .map_err(|e| AppError::EmailError(e.to_string()))?;

        tracing::info!("Passwort-Reset-E-Mail gesendet an: {}", user.email);
        Ok(())
    }
}
