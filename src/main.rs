use std::net::TcpListener;
use actix_web::web::Json;
use sqlx::{PgConnection, Connection, PgPool, postgres::PgPoolOptions};
use env_logger:: Env;
use tracing::subscriber;
use secrecy::ExposeSecret;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[ tokio:: main]
async fn main() -> std::io:: Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(), 
        "info".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);

    // Panic if we can't read the config
    let config = get_configuration().expect("Failed to read config.");
    let connection_pool = PgPoolOptions:: new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
        &config.database.connection_string().expose_secret()
        )
        .expect("Failed to connect to Postgres");

    let address = format!(
        "{}:{}",
        config.application.host, config.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}