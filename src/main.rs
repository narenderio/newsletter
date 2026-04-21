//! src/main.rs
use newsletter::configuration::get_configuration;
use newsletter::email_client::EmailClient;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use reqwest::Url;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsltter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // No longer async, given that we don't actually try to connect!
    let connection_pool = PgPoolOptions::new()
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.connect_options());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");

    let base_url = Url::parse(&configuration.email_client.base_url).expect("Invalid base URL.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        base_url,
        sender_email, // Pass argument from configuration
        configuration.email_client.authorization_token,
        timeout,
    );
    let listener = TcpListener::bind(address)?;
    // New argument for `run`, `email_client`
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
