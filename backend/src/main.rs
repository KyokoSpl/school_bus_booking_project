mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::mysql::MySqlPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use config::Config;
pub use error::{AppError, AppResult};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Umgebungsvariablen laden
    dotenvy::dotenv().ok();

    // Logging initialisieren
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Konfiguration laden
    let config = Config::from_env().expect("Konfiguration konnte nicht geladen werden");
    let config_data = web::Data::new(config.clone());

    // Datenbankverbindung herstellen
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Datenbankverbindung fehlgeschlagen");

    // Migrationen ausführen
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrationen fehlgeschlagen");

    let pool_data = web::Data::new(pool);

    // E-Mail-Service erstellen
    let email_service = services::EmailService::new(&config)
        .expect("E-Mail-Service konnte nicht initialisiert werden");
    let email_data = web::Data::new(email_service);

    tracing::info!("Server startet auf {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(config_data.clone())
            .app_data(pool_data.clone())
            .app_data(email_data.clone())
            .configure(handlers::configure_routes)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
