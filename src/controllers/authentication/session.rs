use crate::controllers::constants::Configuration;
use crate::model::authentication::session::login_database;
use crate::model::structs::LoginCheck;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, http};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_flash_messages::{FlashMessage, FlashMessagesFramework, IncomingFlashMessages};
use actix_web_flash_messages::storage::{CookieMessageStore, SessionMessageStore};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;
use std::fmt::Write;
use actix_web::cookie::Key;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-login-basic", &json!({"one":"one"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    FlashMessage::error("Hey there sadkdmamsdjasndsdnj !").send();
    FlashMessage::debug("How is it going?").send();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
pub struct IncomingFlashMessagess {
    messages: Vec<FlashMessage>,
}

impl IncomingFlashMessagess {
    /// Return an iterator over incoming [`FlashMessage`]s.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &FlashMessage> {
        self.messages.iter()
    }
}
pub async fn login(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
    let mcrypt = &config.magic_key;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    let db = &config.database_connection;
    let login_result = login_database(username, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let logic_check_value = LoginCheck { value: 1 };
    if login_result == logic_check_value {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        FlashMessage::error("error".to_string()).send();
        let html = handlebars
            .render("auth-login-basic", &json!({"one":"one"}))
            .map_err(actix_web::error::ErrorInternalServerError)?;

        FlashMessage::error("Hey there sadkdmamsdjasndsdnj !").send();
        FlashMessage::debug("How is it going?").send();
        let mut error_html = String::new();
        error_html= "login failed".to_string();
        // for m in IncomingFlashMessagess.iter() {
        //     writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
        // }

        Ok(HttpResponse::SeeOther()
               .content_type(ContentType::html())
               .body(error_html))
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(_user) = user {
        web::Redirect::to("/admin/posts/page/1")
    } else {
        web::Redirect::to("/")
    }
}

pub async fn show(messages: IncomingFlashMessages) -> impl Responder {
    let mut body = String::new();
    for message in messages.iter() {
        writeln!(body, "{} - {}", message.content(), message.level()).unwrap();
    }
    HttpResponse::Ok().body(body)
}

pub async fn set() -> impl Responder {
    FlashMessage::error("Hey there sadkdmamsdjasndsdnj !").send();
    FlashMessage::debug("How is it going?").send();
    HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/show"))
        .finish()
}


pub fn build_message_framework(signing_key: Key) -> FlashMessagesFramework {
    let message_store = CookieMessageStore::builder(signing_key).build();
    FlashMessagesFramework::builder(message_store)
        .build()
}
