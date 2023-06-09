use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::fs;

use actix_identity::Identity;
use actix_web::web::Redirect;
use actix_web::{HttpMessage as _, HttpRequest, Responder};

use crate::model::authentication::login_database::{login_database, LoginCheck};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login_page() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/login.hbs").unwrap();
    handlebars
        .register_template_string("login", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("login", &json!({"yy":"uuihiuhuihiuhuih"}))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_data_from_login_page(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
) -> Redirect {
    let username = &form.username;
    let password = &form.password.to_string();

    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);

    let result = login_database(username, encrypted_password).await;
//     let a: Option<Vec<i64>> = Some(LoginCheck[0]);
let b=result.unwrap_or_default() as i64;
//    let c=result.unwrap_or_default().f.cloned().unwrap_or_default() as i64;

    // let y=result.into() as i64;
    // let result = result.0;
     // let result = result::<i32>().unwrap();;
    //let result_inter = result.iter();
// let result = result.iter(1)
// let result1= result as (i64);

    // let result= match result {
    //     Ok(result,) => result,
    //     Err(err) => (1_i64,)
    // };

    let x = 1  as i64;

    if b == x {
        Identity::login(&req.extensions(), username.to_string()).unwrap();
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/login")
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/?page=1")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(_user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")
    }
}
