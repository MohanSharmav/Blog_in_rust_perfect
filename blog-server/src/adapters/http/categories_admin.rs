use crate::adapters::http::auth_guard::require_login;
use crate::adapters::http::flash::render_flash_messages;
use crate::adapters::http::pagination_view::{self, admin_categories};
use crate::adapters::http::state::AppState;
use crate::adapters::http::validated_form::validate_or_redirect;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use blog_core::categories_service;
use blog_storage::domain::category::NewCategory;
use blog_storage::ports::CategoryRepository;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_all_categories(
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let page_number = params.into_inner() as i64;

    let Some(listing) = categories_service::list_categories(&state.categories, page_number)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        return Ok(pagination_view::redirect("/admin/categories/page/1"));
    };

    let pages_count: Vec<_> = (1..=listing.page.total).collect();
    let error_html = render_flash_messages(&flash_message)?;
    let pagination_final_string = admin_categories(listing.page.current, listing.page.total);

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_category_table",
            &json!({"message": error_html,"pagination":pagination_final_string,"z": &listing.items,"o":all_category,"pages_count":pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn new_category(
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let all_category = state
        .categories
        .all()
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
    form: web::Form<NewCategory>,
    state: web::Data<AppState>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    if let Some(redirect) = validate_or_redirect(&*form, "/admin/categories/page/1") {
        return Ok(redirect);
    }

    categories_service::create_category(&state.categories, &form.name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn destroy_category(
    id: web::Path<String>,
    state: web::Data<AppState>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let category_id: i32 = id
        .parse()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    categories_service::delete_category(&state.categories, category_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .finish())
}

pub async fn edit_category(
    state: web::Data<AppState>,
    to_be_updated_category: web::Path<i32>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }

    let to_be_updated_category = *to_be_updated_category;
    let edit_view =
        categories_service::category_for_edit(&state.categories, to_be_updated_category)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category ,"o":edit_view.all_categories,"category_old_name":edit_view.current_name}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_category(
    id: web::Path<i32>,
    form: web::Form<NewCategory>,
    current_category_name: web::Path<String>,
    state: web::Data<AppState>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let _current_post_name = &current_category_name.into_inner();
    let category_id = id.into_inner();

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/categories/page/1") {
        return Ok(redirect);
    }

    categories_service::update_category(&state.categories, category_id, &form.name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}
