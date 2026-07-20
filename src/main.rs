//! Composition root: reads configuration, builds the concrete adapters for
//! each port, wires them into `AppState`, and starts the Actix HTTP server.
//! No business logic lives here — see `domain`, `application`, `adapters`.

mod adapters;
mod application;
mod domain;

use crate::adapters::crypto::MagicCryptCipher;
use crate::adapters::http::auth::build_message_framework;
use crate::adapters::http::routes;
use crate::adapters::http::state::AppState;
use crate::adapters::postgres::category_repository::PgCategoryRepository;
use crate::adapters::postgres::post_repository::PgPostRepository;
use crate::adapters::postgres::user_repository::PgUserRepository;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use handlebars::Handlebars;
use sqlx::postgres::PgPoolOptions;

pub(crate) const COOKIE_DURATION: actix_web::cookie::time::Duration =
    actix_web::cookie::time::Duration::minutes(30);

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let secret_key = Key::generate();
    #[cfg(feature = "cors_for_local_development")]
    let cookie_secure = false;
    #[cfg(not(feature = "cors_for_local_development"))]
    let cookie_secure = true;
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./templates/html/")?;
    handlebars.register_templates_directory(".hbs", "./templates/html/")?;
    dotenv::dotenv()?;
    let magic_key = std::env::var("MAGIC_KEY")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let state = AppState {
        posts: PgPostRepository::new(pool.clone()),
        categories: PgCategoryRepository::new(pool.clone()),
        users: PgUserRepository::new(pool.clone()),
        cipher: MagicCryptCipher::new(&magic_key),
    };
    let state = web::Data::new(state);

    let signing_key = Key::generate();
    let message_framework = build_message_framework(signing_key);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(state.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(message_framework.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("adf-obdd-service-auth".to_owned())
                    .cookie_secure(cookie_secure)
                    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
                    .build(),
            )
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
