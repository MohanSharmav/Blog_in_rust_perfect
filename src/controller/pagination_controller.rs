use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::fs;
use actix_web::http::header::ContentType;

pub async fn pagination_display(
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
    let index_template = fs::read_to_string("templates/pagination_page.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("pagination_page", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let paginators = pagination_logic(params.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let _current_page = params.page;
    let exact_posts_only = select_specific_pages_post(_current_page)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let _html = handlebars.render("pagination_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    let mut handlebarss = handlebars::Handlebars::new();
    let index_templates = fs::read_to_string("templates/admin_page.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebarss
        .register_template_string("admin_page", &index_templates)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebarss.render("admin_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
                .content_type(ContentType::html())

        .body(htmls))
}
// TODO:
// error[E0605]: non-primitive cast: `Result<i64, actix_web::Error>` as `f64`
// --> src/controller/common_controller.rs:10:35
// |
// 10 | ... = perfect_pagination_logic().await as f64;
// |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

pub async fn perfect_pagination_logic() -> Result<i64, actix_web::error::Error> {
    dotenv::dotenv().map_err(actix_web::error::ErrorInternalServerError)?;

    let db_url =
        std::env::var("DATABASE_URL").map_err(actix_web::error::ErrorInternalServerError)?;

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut counting_final: i64 = 0;
    for row in rows {
        let title: i64 = row
            .try_get("count")
            .map_err(actix_web::error::ErrorInternalServerError)?;
        counting_final += title;
    }
    Ok(counting_final)
}

pub async fn category_pagination_logic(category_input: &String) -> Result<i64, anyhow::Error> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await?;

    let category_input = category_input.to_string();
    let category_id = category_input.parse::<i32>()?;

    let rows = sqlx::query("SELECT COUNT(*) FROM posts where category_id=$1")
        .bind(category_id)
        .fetch_all(&pool)
        .await?;

    let mut counting_final: i64 = 0;
    for row in rows {
        let title: i64 = row.try_get("count")?;
        //Todo
        counting_final = title;
    }
    Ok(counting_final)
}
