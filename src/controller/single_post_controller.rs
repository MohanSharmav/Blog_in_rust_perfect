use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::Single_posts_database::query_single_post;

pub async fn get_single_post(path: web::Path<String>) -> HttpResponse
{
    let mut titles=path.into_inner();

// query_single_post(titles.clone()).await.expect("TODO: panic message");
println!("asdsadadsdadadadadadadadadadadadadadad");

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("single", &index_template).expect("TODO: panic message");

    let single_post=query_single_post(titles).await.expect("TODO: panic message");

    let html = handlebars.render("single", &json!({"o":&single_post})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
