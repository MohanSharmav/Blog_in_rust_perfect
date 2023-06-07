use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn common_page_controller(params: web::Query<PaginationParams>) -> HttpResponse {
    let total_posts_length: f64 = perfect_pagination_logic().await as f64;

    let posts_per_page = total_posts_length / 3.0;

    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i64;
    let mut pages_count = Vec::new();
    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/common.hbs").unwrap();
    handlebars
        .register_template_string("common", &index_template)
        .expect("TODO: panic message");

    let paginators = pagination_logic(params.clone()).await.expect("Aasd");

    let current_page = &params.page;
    let exact_posts_only = select_specific_pages_post(current_page)
        .await
        .expect("Aasd");

    let all_category = get_all_categories_database().await.expect("adssad");

    let _html = handlebars.render("common", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;

    let mut handlebarss = handlebars::Handlebars::new();
    let index_templates = fs::read_to_string("templates/common.hbs").unwrap();
    handlebarss
        .register_template_string("common", &index_templates)
        .expect("TODO: panic message");

    let htmls = handlebarss.render("common", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(htmls)
}
