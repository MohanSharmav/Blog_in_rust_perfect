use crate::model::single_posts_database::{query_single_post, query_single_post_in_struct};
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;
// use crate::model::Single_posts_database::{query_single_post, query_single_post_in_struct};

pub async fn get_single_post(path: web::Path<String>) -> HttpResponse {
    let titles = path.parse::<i32>().unwrap();

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/single.hbs").unwrap();
    handlebars
        .register_template_string("single", &index_template)
        .expect("TODO: panic message");

    let single_post = query_single_post(titles)
        .await
        .expect("TODO: panic message");

    let single_post_struct = query_single_post_in_struct(titles)
        .await
        .expect("TODO: panic message");
    let html = handlebars
        .render(
            "single",
            &json!({"o":&single_post,"single_post":single_post_struct}),
        )
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
