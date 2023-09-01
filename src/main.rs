mod controllers;
mod model;
use crate::controllers::admin::categories_controller::{
    create_category, destroy_category, edit_category, get_all_categories, new_category,
    update_category,
};
use crate::controllers::admin::posts_controller::{
    destroy_post, edit_post, get_new_post, new_post, update_post,
};
use crate::controllers::authentication::register::{get_register, register};
use crate::controllers::authentication::session::{
    build_message_framework, check_user, get_login, login, logout,
};
use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::{index, redirect_user};
use crate::controllers::helpers::config::db_config;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use controllers::admin::posts_controller::admin_index;
use controllers::admin::posts_controller::categories_based_posts;
use controllers::guests::posts::{get_category_based_posts, show_post};
use handlebars::Handlebars;
use sqlx::postgres::PgPoolOptions;

pub(crate) const COOKIE_DURATION: actix_web::cookie::time::Duration =
    actix_web::cookie::time::Duration::minutes(30);

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // this will show the rust operation in terminals
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // this secret key is used to send to the user
    // and store in browser cookies
    // key : value
    // id : 0D37VD27Ki9hAvKbQ4u43JonbED8fc%2FnGeIvdpenYL8Yshq+NXqwe8neP6CLZ6gDGbJNJbhVEBG8NiHU4jXH
    // this always generates a unique value
    let secret_key = Key::generate();
    let cookie_secure = true;
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./templates/html/")?;
    let db_url = db_config().await?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;
    let config = Configuration {
        database_connection: pool,
    };
    let configuration = web::Data::new(config.clone());
    let signing_key = Key::generate();
    let message_framework = build_message_framework(signing_key);

    HttpServer::new(move || {
        App::new()
            // in .app_data() you can pass any thing and use
            // it directly in any function like
            // function_name( handlebars: web::Data<Handlebars<'_>>)
            // you need not pass it a parameter when you call a function
            // https://docs.rs/actix-web/latest/actix_web/struct.App.html
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(configuration.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(message_framework.clone())
            // actix session is used to create session and maintain it
            // it is even used to pass cookie also
            // https://docs.rs/actix-session/latest/actix_session/ --> refer this for more information
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(cookie_secure)
                    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
                    .build(),
            )
            .service(web::resource("/").to(redirect_user))
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
                web::resource("/admin/posts/{post_id}/edit")
                    .route(web::get().to(edit_post))
                    .route(web::post().to(update_post)),
            )
            .service(
                web::resource("/admin/post/{post_id}/delete").route(web::get().to(destroy_post)),
            )
            .service(
                web::resource("/admin/categories/{category_id}/page/{page_number}")
                    .to(categories_based_posts),
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
            .service(web::resource("/posts/{post_id}").route(web::get().to(show_post)))
            .service(
                web::resource("/posts/category/{category_id}/page/{page_number}")
                    .to(get_category_based_posts),
            )
            .service(web::resource("/posts/page/{page_number}").route(web::get().to(index)))
            .service(Files::new("/", "./templates").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
