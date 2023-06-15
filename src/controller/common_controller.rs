use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::PgPool;

pub async fn common_page_controller(
    params: web::Query<PaginationParams>,
    db: web::Data<PgPool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let total_posts_length: f64 = perfect_pagination_logic(&db).await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();

    let paginators = pagination_logic(params.clone(), &db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let current_page = params.page;
    let exact_posts_only = select_specific_pages_post(current_page, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(&db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
