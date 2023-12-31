use std::net::TcpListener;
use sqlx::{postgres::PgPoolOptions};
use zero2prod::email_client::EmailClient;
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
        .connect_lazy_with(config.database.with_db());

    let sender_email = config.email_client.sender()
        .expect("Invalid sender email.");
    let email_client = EmailClient::new(
        config.email_client.base_url, 
        sender_email,
    );

    let address = format!(
        "{}:{}",
        config.application.host, config.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}