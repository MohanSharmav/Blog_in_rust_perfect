//! Composition root: reads configuration, builds the concrete adapters for
//! each port, wires them into `AppState`, and starts the Actix HTTP server.
//! No business logic lives here — see `blog-core` (use-case services),
//! `blog-storage` (persistence), `blog-views` (templates/assets), and this
//! crate's own `adapters` (HTTP delivery + crypto).
//!
//! The storage backend is a compile-time choice, not a runtime one: enable
//! exactly one of the `postgres`/`sqlite` Cargo features, which in turn
//! determines the concrete repository types `adapters::http::state::AppState`
//! resolves to. There is deliberately no `DbPool` enum here.

#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("enable exactly one of the `postgres` or `sqlite` features, not both");
#[cfg(not(any(feature = "postgres", feature = "sqlite")))]
compile_error!("enable one of the `postgres` or `sqlite` features (`postgres` is the default)");

mod adapters;

use crate::adapters::http::api::openapi::ApiDoc;
use crate::adapters::http::auth::build_message_framework;
use crate::adapters::http::routes;
use crate::adapters::http::state::AppState;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use handlebars::Handlebars;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    blog_views::register(&mut handlebars)?;
    // Best-effort: a `.env` file is a local-dev convenience. Its absence
    // (e.g. in a container, where the environment is set directly) isn't an
    // error — only actually-missing required variables are, below.
    let _ = dotenv::dotenv();
    let magic_key = std::env::var("MAGIC_KEY")?;
    let db_url = std::env::var("DATABASE_URL")?;

    let state = AppState::connect(&db_url, &magic_key).await?;
    let state = web::Data::new(state);

    let signing_key = Key::generate();
    let message_framework = build_message_framework(signing_key);

    // Defaults to loopback-only for local dev; a container needs 0.0.0.0 to
    // be reachable through Docker's port mapping, so this is runtime-overridable.
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

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
            .configure(crate::adapters::http::api::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(routes::configure)
    })
    .bind(bind_addr)?
    .run()
    .await?;

    Ok(())
}
