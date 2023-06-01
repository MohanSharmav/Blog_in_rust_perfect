use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
};
use actix_web::web::Redirect;
use crate::model::authentication::login_database::login_database;


//extra
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;




// use actix_session::storage::RedisSessionStore;


#[derive(Debug, Clone, PartialEq,Deserialize)]
pub struct user{
    pub(crate) username: String,
    pub(crate) password: String
}
pub async fn get_login_page() -> HttpResponse {
    println!("Welcome to login page");
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/login.hbs").unwrap();
    handlebars
        .register_template_string("login", &index_template).expect("TODO: panic message");


    let html = handlebars.render("login", &json!({"yy":"uuihiuhuihiuhuih"})).unwrap();


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_data_from_login_page(form: web::Form<user>, req: HttpRequest,user: Option<Identity>) -> Redirect
{
println!("🦋");
    check_user(user).await;

 let user = &form.username;
    let password=&form.password.to_string();
// let passwording=
    println!("{}", user);

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");



    //
    // let stored_password = match password {
    //     None => {
    //         let error_message = String::from("Invalid username or password");
    //         return HttpResponse::BadRequest().body(error_message);
    //     },
    //     Some(password) => password,
    // };
    //
    // let stored_hash = PasswordHash::new("asd-asd").unwrap();
    // let pw_valid = Argon2::default()
    //     .verify_password(password.as_bytes(), &stored_hash)
    //     .is_ok();
    //
    // println!("{:?}",pw_valid);
    //
// Todo get the password from register page and send it to database use this
    let hash_secret = "123";
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

println!("🐰🐰🐰🐰🐰{:?}",hash);


        // let stored_hash = PasswordHash::new(password).unwrap();
    // let pw_valid = Argon2::default()
    //     .verify_password(password.as_bytes(), &stored_hash)
    //     .is_ok();
// Todo use this code to get the password from login page and check it with database ... use username to get the password from database
let mut verifier=Verifier::default();
    let is_valid=verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(hash_secret)
        .verify()
        .unwrap() ;


if is_valid{
    println!("🕸️🕸️🕸️🕸️🕸️🕸️🕸️🕸️🕸️pass")
}else {
    println!("fail")
}



let x=login_database(user, password).await;





if(x==1) {

    Identity::login(&req.extensions(), user.to_string()).unwrap();

   // web::Redirect::to("/users?page=1")

    web::Redirect::to("/admin?page=1&limit=2")

    // let success_message="user successfully authenticated";
    // let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;
    //
    //
    // HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(html)
}else{

    web::Redirect::to("/login")

    // let success_message="user successfully authenticated";
    // let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;
    //
    //
    // HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(html)
     // HttpResponse::BadRequest().body("Invalid email or password")

}

}



pub async fn logout(id: Identity) -> impl Responder {
    id.logout();


    //web::Redirect::to("/").using_status_code(StatusCode::FOUND)
    web::Redirect::to("/")
}


pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")

    }
}