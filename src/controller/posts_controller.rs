use crate::model::category_database::get_all_categories_database;
use crate::model::database::Posts;
use crate::model::posts_database::{delete_post_database, update_post_database};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use std::fs;

pub async fn get_new_post(db: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_post.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("new_post", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = get_all_categories_database(&db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_post", &json!({ "all_categories": all_categories }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
pub async fn receive_new_posts(_form: web::Form<Posts>) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "the post created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn delete_post(
    to_delete: web::Path<String>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let to_delete = to_delete.into_inner();
    delete_post_database(to_delete, &db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let success_message = "the post deleted successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn page_to_update_post(
    to_be_updated_post: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_post.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("update_post", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let to_be_updated_post = to_be_updated_post.clone();
    update_post_helper(&to_be_updated_post).await;
    let html = handlebars
        .render(
            "update_post",
            &json!({ "to_be_updated_post": &to_be_updated_post }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_post_helper(ids: &String) -> &String {
    ids
}
pub async fn receive_updated_post(
    form: web::Form<Posts>,
    _current_post_name: web::Path<String>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let id = &form.id;
    let title = &form.title;
    let category_id = &form.category_id;
    let description = &form.description;

    update_post_database(title, description, &id, &category_id, &db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "the post created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
