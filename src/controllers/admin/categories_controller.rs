use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::controllers::helpers::auth_guard::require_login;
use crate::controllers::helpers::flash::render_flash_messages;
use crate::controllers::helpers::pagination_logic::{admin_categories, resolve_current_page};
use crate::controllers::helpers::validated_form::validate_or_redirect;
use crate::model::categories::{
    all_categories_db, create_new_category_db, delete_category_db, get_all_categories_db,
    get_specific_category_posts, update_category_db,
};
use crate::model::structs::CreateNewCategory;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn get_all_categories(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }

    let db = &config.database_connection;
    let total_posts_length = get_pagination_for_all_categories_list(db).await?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let param = params.into_inner();

    let (current_page, count_of_number_of_pages) = match resolve_current_page(
        param as i64,
        total_posts_length,
        posts_per_page_constant,
        false,
        "/admin/categories/page/1",
    ) {
        Ok(resolved) => resolved,
        Err(redirect) => return Ok(redirect),
    };

    let pages_count: Vec<_> = (1..=count_of_number_of_pages).collect();
    let posts_per_page_constant = SET_POSTS_PER_PAGE as i32;
    let error_html = render_flash_messages(&flash_message)?;

    let pagination_final_string = admin_categories(current_page, count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = get_all_categories_db(db, param, posts_per_page_constant)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_category_table",
            &json!({"message": error_html,"pagination":pagination_final_string,"z": &all_categories,"o":all_category,"pages_count":pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn new_category(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let db = &config.database_connection;
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_category", &json!({"o":all_category}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn create_category(
    form: web::Form<CreateNewCategory>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = &form.name;
    let db = &config.database_connection;

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/categories/page/1") {
        return Ok(redirect);
    }

    create_new_category_db(db, name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn destroy_category(
    id: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<Redirect, actix_web::Error> {
    let to_delete_category = &id.into_inner();
    let db = &config.database_connection;
    delete_category_db(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/categories/page/1"))
}

pub async fn edit_category(
    config: web::Data<Configuration>,
    to_be_updated_category: web::Path<i32>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }

    let db = &config.database_connection;
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let to_be_updated_category = *to_be_updated_category;
    let posts = get_specific_category_posts(to_be_updated_category, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category ,"o":all_category,"category_old_name":posts}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_category(
    id: web::Path<i32>,
    form: web::Form<CreateNewCategory>,
    current_category_name: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let _current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let category_id = id.into_inner();

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/categories/page/1") {
        return Ok(redirect);
    }

    update_category_db(name, category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn get_pagination_for_all_categories_list(
    db: &Pool<Postgres>,
) -> Result<i64, actix_web::error::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM categories")
        .fetch_one(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}
