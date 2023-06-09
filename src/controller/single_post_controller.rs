use crate::controller::constants::ConfigurationConstants;
use crate::model::single_posts_database::{query_single_post, query_single_post_in_struct};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_single_post(
    path: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    //Todo
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = query_single_post_in_struct(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "single",
            &json!({"o":&single_post,"single_post":single_post_struct}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
