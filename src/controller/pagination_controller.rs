use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::fs;

pub async fn pagination_display(params: web::Query<PaginationParams>) -> HttpResponse {
    let total_posts_length: f64 = perfect_pagination_logic().await as f64;

    let posts_per_page = total_posts_length / 3.0;

    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i64;
    let mut pages_count = Vec::new();
    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template)
        .expect("TODO: panic message");

    let paginators = pagination_logic(params.clone()).await.expect("Aasd");

    let _current_page = &params.page;
    let exact_posts_only = select_specific_pages_post(_current_page)
        .await
        .expect("Aasd");

    let all_category = get_all_categories_database().await.expect("adssad");

    let _html = handlebars.render("pagination_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;

    let mut handlebarss = handlebars::Handlebars::new();
    let index_templates = fs::read_to_string("templates/admin_page.hbs").unwrap();
    handlebarss
        .register_template_string("admin_page", &index_templates)
        .expect("TODO: panic message");

    let htmls = handlebarss.render("admin_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(htmls)
}

pub async fn perfect_pagination_logic() -> i64 {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut counting_final: i64 = 0;
    for row in rows {
        let title: i64 = row.try_get("count").unwrap();
        counting_final += title;
    }
    counting_final
}

pub async fn category_pagination_logic(category_input: &String) -> i64 {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let category_input = category_input.to_string();
    let category_id = category_input.parse::<i32>().unwrap();

    let rows = sqlx::query("SELECT COUNT(*) FROM posts where category_id=$1")
        .bind(category_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut counting_final: i64 = 0;
    for row in rows {
        let title: i64 = row.try_get("count").unwrap();
        counting_final += title;
    }
    counting_final
}
