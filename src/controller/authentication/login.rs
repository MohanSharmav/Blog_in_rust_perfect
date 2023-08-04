use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginCheck};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;
use std::fs;

// use actix_web_flash::{FlashResponse};
// use actix_web_flash_messages::{
//     FlashMessage{}
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render(
            "auth-login-basic",
            &json!({"p":"home_pageall_posts_in_struct"}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // let html = handlebars
    // .render("templates/sneat-1.0.0/html/auth-login-basic.html",&json!({"yy":"uuihiuhuihiuhuih"}))
    //     .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_data_from_login_page(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
    let mcrypt = &config.magic_key;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    let db = &config.database_connection;
    let result = login_database(username, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let y = LoginCheck { value: 1 };
    if result == y {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        // HttpResponse::Unauthorized().body("Invalid username or password");
        // let  c=HttpResponse::SeeOther()
        //     .insert_header((LOCATION, "/login"))
        //         .status(StatusCode::TEMPORARY_REDIRECT)
        //     .finish();
        // Ok(c)
        // FlashMessage::debug("wrong password").send();
        // FlashMessage::new("How is it going?".to_string(), Level::Debug).send();
        // FlashMessage::success("adasds");
        //   FlashMessage::info("failed to login".to_string()).send();
        // let x = FlashMessage::info("failed to login".to_string()).send();
        // // let flash: Option<FlashMessage> = FlashMessage::extract(&req);
        //
        // #[derive(Serialize)]
        // pub struct TemplateContext {
        //     // Add any other data you want to pass to the template here
        //   pub  flash_messages: Vec<String>,
        // }
        //
        // // let mut data = HashMap::new();
        // // if let Some(flash_msg) = flash {
        // //     let category = flash_msg.category();
        // //     let message = flash_msg.message();
        // //     data.insert("flash_category", category.to_string());
        // //     data.insert("flash_message", message);
        // // }
        // let flash_messages = FlashMessagesMiddleware::get_messages(&req)
        //     .unwrap_or_else(Vec::new);
        //
        // let context = TemplateContext {
        //     flash_messages,
        // };
        //
        //
        // FlashMessagesMiddleware::add_message(&req, "failure", "Form submitted successfully");

        // let login_fail_message = "Wrong Id Or Password".to_string();
        // let html = handlebars
        //     .render("login", &json!({ "flash_message": login_fail_message ,"hello": "world"}))
        //     .map_err(actix_web::error::ErrorInternalServerError)?;
        // println!("--------------------------------😀");
        Ok(HttpResponse::SeeOther()
            // .insert_header(http::header::LOCATION, "/login")
            .insert_header((LOCATION, "/llogin"))
            .content_type(ContentType::html())
            .finish())
        // Ok(HttpResponse::SeeOther()
        //     .insert_header((LOCATION, "/login"))
        //     .finish())
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

pub async fn failed_login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("----------------------------------------------------");
    let html = handlebars
        .render("llogin", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
