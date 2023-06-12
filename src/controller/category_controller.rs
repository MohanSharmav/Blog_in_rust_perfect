use crate::controller::pagination_controller::category_pagination_logic;
use crate::controller::pagination_logic::select_specific_category_post;
use crate::model::category_database::{
    category_pagination_controller_database_function, create_new_category_database,
    delete_category_database, get_all_categories_database, update_category_database,
};
use crate::model::database::Categories;
use crate::model::pagination_database::PaginationParams;
use actix_web::{web, HttpResponse};
use anyhow::Result;
use serde_json::json;
use std::fs;

pub async fn get_all_categories_controller() -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/all_categories.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("all_categories", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = get_all_categories_database()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("all_categories", &json!({ "z": &all_categories }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn get_new_category() -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_category.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("new_category", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_category", &json!({"o":"ax"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn receive_new_category(
    form: web::Form<Categories>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let name = &form.name;
    let id = &form.id;
    create_new_category_database(name, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "the categories created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn delete_category(id: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let to_delete_category = &id.into_inner();
    delete_category_database(to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "the category deleted successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn page_to_update_category(
    to_be_updated_category: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_category.hbs")
        .map_err(|o| actix_web::error::ErrorInternalServerError(o))?;

    handlebars
        .register_template_string("update_category", &index_template)
        .map_err(|o| actix_web::error::ErrorInternalServerError(o))?;

    let to_be_updated_category = to_be_updated_category.clone();

    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category }),
        )
        .map_err(|o| actix_web::error::ErrorInternalServerError(o))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn receive_updated_category(
    form: web::Form<Categories>,
    current_category_name: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let _id = &form.id;
    update_category_database(name, current_post_name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let success_message = "the post created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn get_category_with_pagination(
    path: web::Path<String>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let category_input: String = path.into_inner();
    let total_posts_length = category_pagination_logic(&category_input)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let total_posts_length = total_posts_length as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i64;
    let mut pages_count = Vec::new();
    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/category.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("category", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let current_page = params.page;

    let _exact = select_specific_category_post(current_page, &category_input)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_postinng = category_pagination_controller_database_function(&category_input)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "category",
            &json!({"tiger":&category_postinng,"pages_count":&pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
