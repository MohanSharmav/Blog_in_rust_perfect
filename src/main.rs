mod controller;
mod model;
use crate::controller::admin_function::{admin_category_display, admin_unique_posts_display};
use crate::controller::authentication::login::{
    check_user, get_data_from_login_page, get_login_page, logout,
};
use crate::controller::authentication::register::{get_data_from_register_page, get_register_page};
use crate::controller::category_controller::{
    delete_category, get_all_categories_controller, get_category_with_pagination, get_new_category,
    page_to_update_category, receive_new_category, receive_updated_category,
};
use crate::controller::common_controller::{common_page_controller, redirect_user};
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::pagination_display;
use crate::controller::posts_controller::{
    delete_post, get_new_post, page_to_update_post, receive_new_posts, receive_updated_post,
};
use crate::controller::single_post_controller::get_single_post;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, Result};
use actix_web_lab::middleware::from_fn;
use handlebars::Handlebars;
use magic_crypt::new_magic_crypt;
use sqlx::postgres::PgPoolOptions;
use warp::get;

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
    handlebars.register_templates_directory(".hbs", "./templates/")?;
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

    HttpServer::new(move || {
        App::new()
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
            // .service(web::resource("/post_specific/{title}").to(get_single_post))
            // .service(web::resource("/users").to(pagination_display))
            // .service(web::resource("/").to(common_page_controller))
            // .service(web::resource("/posts/").to(pagination_display))
            // .service(web::resource("/posts/new").to(get_new_post))
            // .service(web::resource("/new_post").route(web::post().to(receive_new_posts)))
            // .service(web::resource("/posts/{title}").to(get_single_post))
            // .service(web::resource("/delete_post/{title}").route(web::get().to(delete_post)))
            // .service(web::resource("/posts/{title}/edit").route(web::get().to(page_to_update_post)))
            // .service(
            //     web::resource("/posts/{title}/edit_complete")
            //         .route(web::post().to(receive_updated_post)),
            // )
            // .service(web::resource("/category").route(web::get().to(get_all_categories_controller)))
            // .service(web::resource("/categories/{name}").to(get_category_with_pagination))
            // .service(web::resource("/category/new").to(get_new_category))
            // .service(
            //     web::resource("/category_received").route(web::post().to(receive_new_category)),
            // )
            // .service(web::resource("/delete_category/{name}").route(web::get().to(delete_category)))
            // .service(web::resource("/posts/{title}").route(web::post().to(receive_updated_post)))
            // .service(
            //     web::resource("/category/{title}/edit")
            //         .route(web::get().to(page_to_update_category)),
            // )
            // .service(
            //     web::resource("/category/{title}").route(web::post().to(receive_updated_category)),
            // )
            .service(web::resource("/").to(redirect_user))
            .service(web::resource("/check").to(check_user))
            // perfect admin url
            .service(web::resource("/admin").to(pagination_display))
            .service(
                web::resource("/admin/categories/new")
                    .route(web::get().to(get_new_category))
                    .route(web::post().to(receive_new_category)),
            )
            //no ui
            .service(
                web::resource("/admin/categories/{title}/edit")
                    .route(web::get().to(page_to_update_category))
                    .route(web::post().to(receive_updated_category)),
            )
            .service(
                web::resource("/admin/categories")
                    .route(web::get().to(get_all_categories_controller)),
            )
            .service(web::resource("/admin/posts/new").to(get_new_post))
            .service(web::resource("/admin/posts").route(web::post().to(receive_new_posts)))
            //change ui
            .service(
                web::resource("/admin/posts/{post_id}")
                    .route(web::get().to(admin_unique_posts_display))
                // .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/admin/posts/{post_id}/edit")
                    .route(web::get().to(page_to_update_post))
                    .route(web::post().to(receive_updated_post))

            )
            .service(web::resource("/admin/post/delete/{post_id}").route(web::get().to(delete_post)))
            //change ui
            .service(web::resource("/admin/categories/{category_id}").to(admin_category_display))
            .service(
                web::resource("/admin/delete_category/{name}")
                    .route(web::get().to(delete_category)),
            )
            // .service(web::resource("/admin/category/{category_id}").route(web::get().to(admin_category_display)))
            //change ui
            //Login
            .service(
                web::resource("/login")
                    .route(web::get().to(get_login_page))
                    .route(web::post().to(get_data_from_login_page)),
            )
            .service(web::resource("/logout").to(logout))
            .service(
                web::resource("/register")
                    .route(web::get().to(get_register_page))
                    .route(web::post().to(get_data_from_register_page)),
            )
            //public
            .service(web::resource("/posts").route(web::get().to(common_page_controller)))
            .service(web::resource("/posts/{post_id}").route(web::get().to(get_single_post)))
            //below two is not working
            .service(
                web::resource(" /posts/page/{page_number}")
                    .route(web::get().to(common_page_controller)),
            )
            .service(
                web::resource("/posts/category/{category_id}").to(get_category_with_pagination),
            )

        // .wrap(from_fn(check_user)
        // .service(web::scope("/admin")
        //     .wrap(from_fn(check_user)
        //               .route("/dashboard", web::get()
        //                   .to(pagination_display))
        //     ))

        // .service(web::resource("/admin").to(pagination_display))
        // .service(
        //     web::scope("/admin")
        //         .wrap(from_fn(reject_anonymous_users))
        //         .route("/dashboard", web::get().to(pagination_display)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
