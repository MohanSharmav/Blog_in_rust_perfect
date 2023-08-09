use actix_web::{HttpResponse, web};
use serde_json::json;
use actix_web::http::header::ContentType;
use handlebars::Handlebars;

pub async fn get_login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-login-basic", &json!({"m":"ASs"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
