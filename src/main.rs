mod controllers;
mod model;
use crate::controllers::admin::categories_controller::{
    create_category, destroy_category, edit_category, get_all_categories, get_category_posts,
    new_category, update_category,
};
use crate::controllers::admin::posts_controller::{
    destroy_post, edit_post, get_new_post, new_post, update_post,
};
use crate::controllers::authentication::register::{get_register, register};
use crate::controllers::authentication::session::{check_user, get_login, login, logout};
use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::{index, index_redirect, redirect_user};
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use controllers::admin::posts_controller::admin_index;
use controllers::admin::posts_controller::{get_categories_posts, show_post};
use controllers::guests::posts::show_posts;
use handlebars::Handlebars;
use magic_crypt::new_magic_crypt;
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
    let value = std::env::var("MAGIC_KEY")?;
    let mcrypt = new_magic_crypt!(value, 256); //Creates an instance of the magic crypt library/crate.
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let config = Configuration {
        magic_key: mcrypt,
        database_connection: pool,
    };
    let confi = web::Data::new(config.clone());

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    HttpServer::new(move || {
        App::new()
            .app_data(message_framework.clone())
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(confi.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("adf-obdd-service-auth".to_owned())
                    .cookie_secure(cookie_secure)
                    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
                    .build(),
            )
            .service(web::resource("/").to(redirect_user))
            .service(web::resource("/posts").to(index_redirect))
            .service(web::resource("./templates/").to(redirect_user))
            .service(web::resource("/check").to(check_user))
            .service(web::resource("/admin/posts/page/{page_number}").to(admin_index))
            .service(
                web::resource("/admin/categories/new")
                    .route(web::get().to(new_category))
                    .route(web::post().to(create_category)),
            )
            .service(
                web::resource("/admin/category/{title}/edit")
                    .route(web::get().to(edit_category))
                    .route(web::post().to(update_category)),
            )
            .service(
                web::resource("/admin/categories/page/{page_number}")
                    .route(web::get().to(get_all_categories)),
            )
            .service(web::resource("/admin/posts/new").to(get_new_post))
            .service(web::resource("/admin/posts").route(web::post().to(new_post)))
            .service(
                web::resource("/admin/posts/{post_id}").route(web::get().to(show_post)), // .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/admin/posts/{post_id}/edit")
                    .route(web::get().to(edit_post))
                    .route(web::post().to(update_post)),
            )
            .service(
                web::resource("/admin/post/{post_id}/delete").route(web::get().to(destroy_post)),
            )
            .service(
                web::resource("/admin/categories/{category_id}/page/{page_number}")
                    .to(get_categories_posts),
            )
            .service(
                web::resource("/admin/category/{name}/delete")
                    .route(web::get().to(destroy_category)),
            )
            .service(
                web::resource("/login")
                    .route(web::get().to(get_login))
                    .route(web::post().to(login)),
            )
            .service(web::resource("/logout").to(logout))
            .service(
                web::resource("/register")
                    .route(web::get().to(get_register))
                    .route(web::post().to(register)),
            )
            .service(web::resource("/posts/{post_id}").route(web::get().to(show_posts)))
            .service(
                web::resource("/posts/category/{category_id}/page/{page_number}")
                    .to(get_category_posts),
            )
            .service(web::resource("/posts/page/{page_number}").route(web::get().to(index)))
            .service(Files::new("/", "./templates").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
