mod controller;
mod model;

use crate::controller::admin_function::{admin_category_display, admin_unique_posts_display};
use crate::controller::authentication::login::{
    check_user, failed_login_page, get_data_from_login_page, get_login_page, logout,
};
use crate::controller::authentication::register::{get_data_from_register_page, get_register_page};
use crate::controller::category_controller::{
    delete_category, get_all_categories_controller, get_category_with_pagination, get_new_category,
    page_to_update_category, receive_new_category, receive_updated_category,
};
use crate::controller::common_controller::{
    main_page, new_common_page_controller, new_common_page_controller_test, redirect_user,
};
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::{
    admin_pagination_display, england_admin_pagination_display,
};
use crate::controller::posts_controller::{
    delete_post, get_new_post, page_to_update_post, receive_new_posts, receive_updated_post,
};
use crate::controller::single_post_controller::get_single_post;
use std::fs::DirEntry;
use std::path::Path;
// use crate::controller::test_liquid::{liquid, visit_dirs};
use crate::model::authentication::login_database::login_database;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
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

    let config = ConfigurationConstants {
        magic_key: mcrypt,
        database_connection: pool,
    };
    let confi = web::Data::new(config.clone());
    // let message_store = CookieMessageStore::builder(secret_key.clone()).build();

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    // let files = Files::new("/assets/vendor/css/pages", "./templates/sneat-1.0.0/assets/vendor/css/pages").show_files_listing();

    // let path = Path::new("templates/sneat-1.0.0");
    // let cb = |entry: &DirEntry| {
    //     println!("{}", entry.path().display());
    // };
    // visit_dirs(&path, &cb).unwrap();
    //

    //
    // let signing_key = Key::generate(); // This will usually come from configuration!
    // let message_store = CookieMessageStore::builder(signing_key).build();
    // let message_framework = FlashMessagesFramework::builder(message_store).build();
    //

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(confi.clone())
            .wrap(message_framework.clone())
            .wrap(IdentityMiddleware::default())
            // .wrap(FlashMiddleware::default())
            // .wrap(message_framework.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("adf-obdd-service-auth".to_owned())
                    .cookie_secure(cookie_secure)
                    .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
                    .build(),
            )
            .service(web::resource( "/").to(redirect_user))
            .service(web::resource("/posts").to(main_page))
            .service(web::resource("./templates/").to(redirect_user))
            .service(web::resource("/check").to(check_user))
            // perfect admin url
            // .service(web::resource("/admin/posts/page/{page_number}").to(admin_pagination_display))
            .service(web::resource("/admin/posts/page/{page_number}").to(admin_pagination_display))
            .service(
                web::resource("/admin/categories/new")
                    .route(web::get().to(get_new_category))
                    .route(web::post().to(receive_new_category)),
            )
            .service(
                web::resource("/admin/category/{title}/edit")
                    .route(web::get().to(page_to_update_category))
                    .route(web::post().to(receive_updated_category)),
            )
            .service(
                web::resource("/admin/categories/page/{page_number}")
                    .route(web::get().to(get_all_categories_controller)),
            )
            .service(web::resource("/admin/posts/new").to(get_new_post))
            // .service(web::resource("/admin/posts/new").to(receive_new_posts_with_no_category))
            .service(web::resource("/admin/posts").route(web::post().to(receive_new_posts)))
            .service(
                web::resource("/admin/posts/{post_id}")
                    .route(web::get().to(admin_unique_posts_display)), // .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/admin/posts/{post_id}/edit")
                    .route(web::get().to(page_to_update_post))
                    .route(web::post().to(receive_updated_post)),
            )
            .service(
                web::resource("/admin/post/{post_id}/delete").route(web::get().to(delete_post)),
            )
            .service(
                web::resource("/admin/categories/{category_id}/page/{page_number}")
                    .to(admin_category_display),
            )
            .service(
                web::resource("/admin/category/{name}/delete")
                    .route(web::get().to(delete_category)),
            )
            .service(
                web::resource("/login")
                    .route(web::get().to(get_login_page))
                    .route(web::post().to(get_data_from_login_page)),
            )
            .service(web::resource("/llogin").route(web::get().to(failed_login_page)))
            .service(web::resource("/logout").to(logout))
            .service(
                web::resource("/register")
                    .route(web::get().to(get_register_page))
                    .route(web::post().to(get_data_from_register_page)),
            )
            // .service(web::resource("/posts").route(web::get().to(common_page_controller)))
            .service(web::resource("/posts/{post_id}").route(web::get().to(get_single_post)))
            .service(
                web::resource("/posts/category/{category_id}/page/{page_number}")
                    .to(get_category_with_pagination),
            )
            .service(
                web::resource("/posts/page/{page_number}")
                    .route(web::get().to(new_common_page_controller)),
            )
            .service(
                web::resource("/posts/ben/{page_number}")
                    .route(web::get().to(new_common_page_controller_test)),
            )
            // .service(web::resource("/test").route(web::get().to(new_test)))
            .service(web::resource("/ben").to(england_admin_pagination_display))
            .service(Files::new("/",
                                "./templates").show_files_listing())

        // .service(Files::new("/admin",
        //                     "./templates")
        //     .show_files_listing())

        // .service(Files::new("/",
        //                     "./templates")
        //     .show_files_listing())
        // .service(Files::new("/admin/assets",
        //                     "./templates/sneat-1.0.0").show_files_listing())
        // .service(Files::new(".../",
        //                     "./templates/sneat-1.0.0").show_files_listing())

        // .service(Files::new("/assets",
        //                     "./templates")
        //     .show_files_listing())
        //
        // .service(Files::new("/admin/posts",
        //                     "./templates/sneat-1.0.0")
        //     .show_files_listing())

        // .service(web::resource("/posts/"))
        // .service(web::resource("/{username}/{id}").route(web::get().to(index)))

        // .service(web::resource("/new").to(liquid))
        // .service(web::resource("/{*}").to(get_login_page))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
