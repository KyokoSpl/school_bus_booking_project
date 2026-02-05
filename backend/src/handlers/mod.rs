pub mod auth;
pub mod trips;
pub mod bookings;
pub mod users;
pub mod buses;
pub mod payments;
pub mod dashboard;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Öffentliche Auth-Routen
        .service(
            web::scope("/api/auth")
                .route("/register", web::post().to(auth::register))
                .route("/login", web::post().to(auth::login))
                .route("/verify-email", web::get().to(auth::verify_email))
                .route("/forgot-password", web::post().to(auth::forgot_password))
                .route("/reset-password", web::post().to(auth::reset_password))
        )
        // Geschützte Benutzer-Routen
        .service(
            web::scope("/api/user")
                .route("/profile", web::get().to(auth::get_profile))
                .route("/profile", web::put().to(auth::update_profile))
                .route("/change-password", web::post().to(auth::change_password))
        )
        // Öffentliche Reise-Routen
        .service(
            web::scope("/api/trips")
                .route("", web::get().to(trips::list_trips))
                .route("/search", web::get().to(trips::search_trips))
                .route("/{id}", web::get().to(trips::get_trip))
        )
        // Geschützte Buchungs-Routen
        .service(
            web::scope("/api/bookings")
                .route("", web::get().to(bookings::list_user_bookings))
                .route("", web::post().to(bookings::create_booking))
                .route("/{id}", web::get().to(bookings::get_booking))
                .route("/{id}/cancel", web::post().to(bookings::cancel_booking))
        )
        // Zahlungs-Routen
        .service(
            web::scope("/api/payments")
                .route("/initiate", web::post().to(payments::initiate_payment))
                .route("/{id}", web::get().to(payments::get_payment))
        )
        // Admin-Routen
        .service(
            web::scope("/api/admin")
                // Dashboard
                .route("/dashboard", web::get().to(dashboard::get_dashboard_stats))
                // Reisen
                .route("/trips", web::get().to(trips::admin_list_trips))
                .route("/trips", web::post().to(trips::create_trip))
                .route("/trips/{id}", web::put().to(trips::update_trip))
                .route("/trips/{id}", web::delete().to(trips::delete_trip))
                // Buchungen
                .route("/bookings", web::get().to(bookings::admin_list_bookings))
                .route("/bookings/{id}", web::get().to(bookings::admin_get_booking))
                .route("/bookings/{id}", web::put().to(bookings::admin_update_booking))
                // Benutzer
                .route("/users", web::get().to(users::list_users))
                .route("/users/{id}", web::get().to(users::get_user))
                .route("/users", web::post().to(users::create_user))
                .route("/users/{id}", web::put().to(users::update_user))
                .route("/users/{id}", web::delete().to(users::delete_user))
                // Busse
                .route("/buses", web::get().to(buses::list_buses))
                .route("/buses/{id}", web::get().to(buses::get_bus))
                .route("/buses", web::post().to(buses::create_bus))
                .route("/buses/{id}", web::put().to(buses::update_bus))
                .route("/buses/{id}", web::delete().to(buses::delete_bus))
        )
        // Health-Check
        .route("/api/health", web::get().to(health_check));
}

async fn health_check() -> &'static str {
    "OK"
}
