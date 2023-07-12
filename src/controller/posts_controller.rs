use crate::controller::constants::ConfigurationConstants;
use crate::model::category_database::get_all_categories_database;
use crate::model::database::{CreatePost, Posts};
use crate::model::posts_database::{create_post_database, delete_post_database, update_post_database};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_new_post(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let all_categories = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_post", &json!({ "all_categories": all_categories }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
pub async fn receive_new_posts(
    form: web::Form<CreatePost>,
    handlebars: web::Data<Handlebars<'_>>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let id=form.id as usize;
    let title =&form.title;
    let description=&form.description;
    let category_id=&form.category_id;
    create_post_database(id, title.clone(), description.clone(), category_id, db)
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

pub async fn delete_post(
    to_delete: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("----------------------------------------------------------------");
    let db = &config.database_connection;
    let to_delete = to_delete.into_inner();
    delete_post_database(to_delete, db)
        .await
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
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
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
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let id = &form.id;
    let title = &form.title;
    let description = &form.description;

    update_post_database(title, description, &id, db)
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
