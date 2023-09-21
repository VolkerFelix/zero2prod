use crate::routes::{health_check, subscribe};

use actix_web::{web, App, HttpServer};
use sqlx::{PgPool};
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use tracing_actix_web:: TracingLogger;

pub fn run(f_listener: TcpListener, f_db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(f_db_pool);
    let server = HttpServer::new( move || {
            App::new()
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                // Register the connection as part of the application state
                .app_data(db_pool.clone())
        })
        .listen(f_listener)?
        .run();
    Ok(server)
}