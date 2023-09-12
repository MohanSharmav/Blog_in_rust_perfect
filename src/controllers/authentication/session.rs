use crate::controllers::constants::Configuration;
use crate::model::authentication::session::{password_check, User};
use actix_identity::Identity;
use actix_web::cookie::Key;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{http, web, HttpResponse};
use actix_web::{HttpMessage, HttpRequest, Responder};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::{FlashMessage, FlashMessagesFramework, IncomingFlashMessages};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;
use std::borrow::Borrow;
use web_sys::Storage;

pub async fn get_login(
    handlebars: web::Data<Handlebars<'_>>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    // check any flash messages have been received
    // iterate through flash messages and add it to the string
    flash_message
        .iter()
        .for_each(|message| error_html.push_str(&*message.content().to_string()));

    let html = handlebars
        .render("auth-login-basic", &json!({ "message": error_html }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn login(
    form: web::Form<User>,
    req: HttpRequest,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = form.into_inner();
    let username = user.password;
    let password_input = user.username;
    // let password_input = &form.into_inner().password;
    let db = &config.database_connection;

    let parsed_hash = password_check(&username, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // if no user exists then null will be returned from DB
    // None will be returned in option if the user does not exist
    // check is no user exits then throw a flash message
    if parsed_hash.is_none() {
        FlashMessage::error("Login Fail - Wrong Id or password!").send();

        Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/login"))
            .finish())
    } else {
        let struct_only = parsed_hash.unwrap_or_default();
        let PasswordStruct { password } = struct_only;
        // insert password from DB to password hash
        let parsed_stored =
            PasswordHash::new(&password).map_err(actix_web::error::ErrorInternalServerError)?;
        // check the user password and check the password from database
        let valid_user = Argon2::default()
            .verify_password(password_input.as_bytes(), parsed_stored.borrow())
            .map_err(actix_web::error::ErrorInternalServerError);
        // check the verify_password is successful
        // or failure using match
        match valid_user {
            Ok(_) => {
                // session.insert("counter", count + 1)?;

                Identity::login(&req.extensions(), username)
                    .map_err(actix_web::error::ErrorInternalServerError)?;

                Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/admin/posts/page/1"))
                    .finish())
            }
            Err(_) => {
                FlashMessage::error("Login Fail - Wrong Id or password!").send();

                Ok(HttpResponse::SeeOther()
                    .insert_header((http::header::LOCATION, "/login"))
                    .finish())
            }
        }
    }
}
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    // check of the user exists by using cookie from browser
    if let Some(_user) = user {
        web::Redirect::to("/admin/posts/page/1")
    } else {
        web::Redirect::to("/")
    }
}
pub fn build_message_framework(signing_key: Key) -> FlashMessagesFramework {
    // send flash messages using cookies
    // --> CookieMessageStore will send the data to the browser
    let message_store = CookieMessageStore::builder(signing_key).build();
    FlashMessagesFramework::builder(message_store).build()
}

#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct PasswordStruct {
    pub password: String,
}

impl Default for PasswordStruct {
    fn default() -> Self {
        PasswordStruct {
            password: "password".to_string(),
        }
    }
}
