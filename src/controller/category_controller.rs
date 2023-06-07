use crate::controller::pagination_controller::category_pagination_logic;
use crate::controller::pagination_logic::select_specific_category_post;
use crate::model::category_database::{
    category_pagination_controller_database_function, create_new_category_database,
    delete_category_database, get_all_categories_database, update_category_database,
};
use crate::model::database::Categories;
use crate::model::pagination_database::PaginationParams;
use actix_web::{web, HttpResponse};
use serde_json::json;
use std::fs;

pub async fn get_all_categories_controller() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/all_categories.hbs").unwrap();
    handlebars
        .register_template_string("all_categories", &index_template)
        .expect("TODO: panic message");

    let all_categories = get_all_categories_database()
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render("all_categories", &json!({ "z": &all_categories }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_new_category() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_category.hbs").unwrap();
    handlebars
        .register_template_string("new_category", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("new_category", &json!({"o":"ax"}))
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn receive_new_category(form: web::Form<Categories>) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template)
        .expect("TODO: panic message");

    let name = &form.name;
    let id = &form.id;

    create_new_category_database(name, id)
        .await
        .expect("TODO: panic message");
    let success_message = "the categories created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn delete_category(id: web::Path<String>) -> HttpResponse {
    let to_delete_category = &id.into_inner();

    delete_category_database(to_delete_category)
        .await
        .expect(" panic message");
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template)
        .expect("TODO: panic message");
    let success_message = "the category deleted successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn page_to_update_category(to_be_updated_category: web::Path<String>) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_category.hbs").unwrap();
    handlebars
        .register_template_string("update_category", &index_template)
        .expect("TODO: panic message");

    let to_be_updated_category = to_be_updated_category.clone();

    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category }),
        )
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn receive_updated_category(
    form: web::Form<Categories>,
    current_category_name: web::Path<String>,
) -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template)
        .expect("TODO: panic message");

    let current_post_name = &current_category_name.into_inner();

    let name = &form.name;
    let _id = &form.id;

    update_category_database(name, current_post_name)
        .await
        .expect("TODO: panic message");
    let success_message = "the post created successfully";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_category_with_pagination(
    path: web::Path<String>,
    params: web::Query<PaginationParams>,
) -> HttpResponse {
    let category_input: String = path.into_inner();
    let total_posts_length: f64 = category_pagination_logic(&category_input).await as f64;

    let posts_per_page = total_posts_length / 3.0;

    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i64;
    let mut pages_count = Vec::new();
    for i in 0..posts_per_page {
        pages_count.push(i + 1_i64);
    }

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template)
        .expect("TODO: panic message");

    let current_page = &params.page;

    let _exact = select_specific_category_post(current_page, &category_input)
        .await
        .expect("Aasd");

    let category_postinng = category_pagination_controller_database_function(&category_input)
        .await
        .expect("TODO: panic message");

    let html = handlebars
        .render(
            "category",
            &json!({"tiger":&category_postinng,"pages_count":&pages_count}),
        )
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
