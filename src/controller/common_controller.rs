use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn common_page_controller(
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let total_posts_length: f64 = perfect_pagination_logic().await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i64;
    let mut pages_count = Vec::new();
    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/common.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("common", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let paginators = pagination_logic(params.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let current_page = params.page;
    let exact_posts_only = select_specific_pages_post(current_page)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut handlebarss = handlebars::Handlebars::new();
    let index_templates = fs::read_to_string("templates/common.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebarss
        .register_template_string("common", &index_templates)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebarss.render("common", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
