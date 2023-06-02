use std::fmt::Error;
use std::fs;
use std::ptr::hash;
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
// use crate::model::authentication::login_database::login_database;


//extra
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use secrecy::{ExposeSecret, Secret, SecretVec};
use sha2::Sha256;
use crate::model::authentication::login_database::login_database;



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
     let username = &form.username;
    let password=&form.password.to_string();

     let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
     let encrypted_password = mcrypt.encrypt_str_to_base64( password);


     let mut result=login_database(username,encrypted_password).await;
     println!("result is--------------------------------{:?}",result);
let result = result.0 as i64;
let x=1 as i64;

     if(result==x) {
    Identity::login(&req.extensions(), username.to_string()).unwrap();
    web::Redirect::to("/admin?page=1&limit=2")

}else{

    web::Redirect::to("/login")
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


// test
pub fn check_Encryption()  {
    let str="england cricket";
    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let encrypted_string = mcrypt.encrypt_str_to_base64( str); //Encrypts the string and saves it to the 'encrypted_string' variable.
    println!("🐯🐯🐯🐯🐯Encrypted String: {}", encrypted_string); //Pr
    let decrypted_string = mcrypt.decrypt_base64_to_string(&encrypted_string).unwrap(); //Decrypts the string so we can read it.
    println!("Decrypted String: {}", decrypted_string); //P
}
