use crate::adapters::http::flash::render_flash_messages;
use crate::adapters::http::state::AppState;
use crate::application::auth_service;
use crate::domain::credentials::Credentials;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::cookie::Key;
use actix_web::http::header::ContentType;
use actix_web::{http, web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::{FlashMessage, FlashMessagesFramework, IncomingFlashMessages};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_login(
    handlebars: web::Data<Handlebars<'_>>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let error_html = render_flash_messages(&flash_message)?;
    let html = handlebars
        .render("auth-login-basic", &json!({ "message": error_html }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn login(
    form: web::Form<Credentials>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let authenticated =
        auth_service::authenticate(&state.users, &state.cipher, &form.username, &form.password)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    if authenticated {
        Identity::login(&req.extensions(), form.username.clone())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        FlashMessage::error("Login Fail - Wrong Id or password!").send();
        Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/login"))
            .finish())
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if user.is_some() {
        web::Redirect::to("/admin/posts/page/1")
    } else {
        web::Redirect::to("/")
    }
}

pub fn build_message_framework(signing_key: Key) -> FlashMessagesFramework {
    let message_store = CookieMessageStore::builder(signing_key).build();
    FlashMessagesFramework::builder(message_store).build()
}

pub async fn get_register(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-register-basic", &json!({"yy":"welcome"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn register(
    form: web::Form<Credentials>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    auth_service::register(&state.users, &state.cipher, &form.username, &form.password)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
}
