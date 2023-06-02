mod model;
mod controller;

use std::env::Args;
use std::fmt::{Debug, Error, Formatter};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, Responder, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use controller::home_page::get_all_posts;
use model::database::get_all_categories;
use warp::{get, Rejection, Reply, service};
use crate::controller::authentication::login::{ check_user, get_data_from_login_page, get_login_page, logout};
// use crate::controller::authentication::Auth::{index, login, logout};
use crate::controller::category_controller::{specific_category_controller, delete_category, get_new_category, receive_new_category, get_all_categories_controller, page_to_update_category, receive_updated_category};
use crate::controller::pagination_controller::{pagination_display, perfect_pagination_logic};
use crate::controller::posts_controller::{delete_post, get_new_post, page_to_update_post, receive_new_posts, receive_updated_post};
use crate::controller::single_post_controller::get_single_post;
use crate::model::database::{select_all_from_table};
use crate::model::pagination_database::{ pagination_logic};



use actix_web::cookie::Key;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};

use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use crate::controller::authentication::register::{get_data_from_register_page, get_register_page};


// async fn index(req: HttpRequest)->Responder<Body=()> {
//      println!("🏏🏏🏏🏏");
// }

pub(crate) const COOKIE_DURATION: actix_web::cookie::time::Duration =
     actix_web::cookie::time::Duration::minutes(30);

#[tokio::main]
async fn main() -> Result<()>{
     let secret_key = Key::generate();

// check_Encryption();

//.wrap(IdentityMiddleware::default())
     #[cfg(feature = "cors_for_local_development")]
         let cookie_secure = false;
     #[cfg(not(feature = "cors_for_local_development"))]
         let cookie_secure = true;
     HttpServer::new(move|| {
          App::new()
              .wrap(IdentityMiddleware::default())
              .wrap(SessionMiddleware::builder(CookieSessionStore::default(),secret_key.clone())
              .cookie_name("adf-obdd-service-auth".to_owned())
              .cookie_secure(cookie_secure)
              .session_lifecycle(PersistentSession::default().session_ttl(COOKIE_DURATION))
              .build()
          )
              .service(web::resource("/").to(get_all_posts))
              .service(web::resource("/post_specific/{title}").to(get_single_post))
              .service(web::resource("/users").to(pagination_display))


//posts
              .service(web::resource("/posts").to(pagination_display))
              .service(web::resource("/posts/new").to(get_new_post))
              .service(web::resource("/new_post").route(web::post().to(receive_new_posts)))
              .service(web::resource("/posts/{title}").to(get_single_post))

              // Todo change delete_post to the delete method
              .service(web::resource("/delete_post/{title}").route(web::get().to(delete_post)))


              .service(web::resource("/posts/{title}/edit").route(web::get().to(page_to_update_post)))
              .service(web::resource("/posts/{title}/edit_complete").route(web::post().to(receive_updated_post)))
              // .service(web::resource("/posts/{title}").route(web::delete().to(receive_updated_post)))


              // category
//todo create a route /category get all the categories
//  .service(web::resource("/ca").route(web::get().to(get_all_categories_controller())))
              //   .service(web::resource("/c").route(web::get().to(get_all_categories_controller)))
              .service(web::resource("/category").route(web::get().to(get_all_categories_controller)))
              .service(web::resource("/categories/{name}").to(specific_category_controller))
              .service(web::resource("/category/new").to(get_new_category))
              .service(web::resource("/category_received").route(web::post().to(receive_new_category)))
              // Todo change delete_post to the delete method and url to --> /category/{name}
              .service(web::resource("/delete_category/{name}").route(web::get().to(delete_category)))

              .service(web::resource("/posts/{title}").route(web::post().to(receive_updated_post)))

              .service(web::resource("/category/{title}/edit").route(web::get().to(page_to_update_category)))
              .service(web::resource("/category/{title}").route(web::post().to(receive_updated_category)))

              // Authentication
              //
              //    .service(web::resource("/").to(index))

              // .wrap(IdentityMiddleware::default())
              .service(web::resource("/admin").to(pagination_display))
              .service(web::resource("/login").to(get_login_page))
              .service(web::resource("/login-success").route(web::post().to(get_data_from_login_page)))
              .service(web::resource("/logout").to(logout))

              .service(web::resource("/register").to(get_register_page))
              .service(web::resource("/register-successful").route(web::post().to(get_data_from_register_page)))

              .service(web::resource("/check").to(check_user))
          //
//      let secret_key = Key::generate();
//
//   //   let secret_key = Secret::new("my-secret");
//      // let encrypted_data = secrecy::encrypt(secret, "my-data");
//
//      #[cfg(feature = "cors_for_local_development")]
//          let cookie_secure = false;
//      #[cfg(not(feature = "coe("/posts/{title}").route(web::delete().to(receive_updated_post)))
//
//
//               // category
// //todo create a route /category get all the categories
// //  .service(web::resource("/ca").route(web::get().to(get_all_categories_controller())))
//              .service(web::resource("/category").route(web::get().to(get_all_categories_controller)))
//               .service(web::resource("/categories/{name}").to(specific_category_controller))
//               .service(web::resource("/category/new").to(get_new_category))
//                   .service(web::resource("/category").route(web::post().to(receive_new_category)))
//               // Todo change delete_post to the delete method and url to --> /category/{name}
//               .service(web::resource("/delete_category/{name}").route(web::get().to(delete_category)))
//               .service(web::resource("/posts/{title}").route(web::post().to(receive_updated_post)))
//               .service(web::resource("/category/{title}/edit").route(web::get().to(page_to_update_post)))
//               .service(web::resource("/category/{title}"))
//
//           // Authentication
//           //
//            //    .service(web::resource("/").to(index))
//
//               .wrap(IdentityMiddleware::default())
//                .service(web::resource("/login").to(get_login_page))
//              .service(web::resource("/login-success").route(web::post().to(get_data_from_login_page)))
//               .service(web::resource("/logout").to(logout))
//
//               .service(web::resource("/register").to(get_register_page))
//               .service(web::resource("/register-successful").route(web::post().to(get_data_from_register_page)))
//
//           //admin only

     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
