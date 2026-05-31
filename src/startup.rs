use crate::email_client::EmailClient;

use crate::routes::{
    admin_dashboard, change_password, change_password_form, confirm, health_check, home, log_out,
    login, login_form, publish_newsletter, publish_newsletter_form, subscribe,
};
use actix_session::storage::RedisSessionStore;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer, web};

use secrecy::Secret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// New imports!/ New imports!
use crate::configuration::{DatabaseSettings, Settings};
use sqlx::postgres::PgPoolOptions;

use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::CookieMessageStore;
use secrecy::ExposeSecret;

use crate::authentication::reject_anonymous_users;
use actix_web::middleware::from_fn;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            email_client,
            // New parameter!
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
        )
        .await?;

        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose us to conflicts.
pub struct ApplicationBaseUrl(pub String);

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    // New parameter!
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .service(
                web::scope("/admin")
                    .wrap(from_fn(reject_anonymous_users))
                    .route("/dashboard", web::get().to(admin_dashboard))
                    .route("/newsletters", web::get().to(publish_newsletter_form))
                    .route("/newsletters", web::post().to(publish_newsletter))
                    .route("/password", web::get().to(change_password_form))
                    .route("/password", web::post().to(change_password))
                    .route("/logout", web::post().to(log_out)),
            )
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
