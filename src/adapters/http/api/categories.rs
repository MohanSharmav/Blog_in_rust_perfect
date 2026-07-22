#[allow(unused_imports)]
use crate::adapters::http::api::openapi;
use crate::adapters::http::api::{require_login, ApiError, PageResponse};
use crate::adapters::http::state::AppState;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use blog_core::categories_service;
use blog_storage::domain::category::NewCategory;
use blog_storage::ports::CategoryRepository;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
}

/// List categories (paginated, 3 per page).
#[utoipa::path(
    get,
    path = "/api/v1/categories",
    tag = "categories",
    params(("page" = Option<i64>, Query, description = "1-indexed page number, default 1")),
    responses(
        (status = 200, body = openapi::PageOfCategories),
        (status = 404, description = "Page out of range", body = openapi::ApiError),
    )
)]
pub async fn list(
    query: web::Query<PageQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = query.page.unwrap_or(1);
    match categories_service::list_categories(&state.categories, page)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(listing) => Ok(HttpResponse::Ok().json(PageResponse::from(listing))),
        None => Ok(HttpResponse::NotFound().json(ApiError {
            error: "page out of range".to_string(),
        })),
    }
}

/// Get a single category by id.
#[utoipa::path(
    get,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    params(("id" = i32, Path, description = "Category id")),
    responses(
        (status = 200, body = openapi::Category),
        (status = 404, description = "Category not found", body = openapi::ApiError),
    )
)]
pub async fn get(
    path: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let categories = state
        .categories
        .find(path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match categories.into_iter().next() {
        Some(category) => Ok(HttpResponse::Ok().json(category)),
        None => Ok(HttpResponse::NotFound().json(ApiError {
            error: "category not found".to_string(),
        })),
    }
}

/// Create a category. Requires login.
#[utoipa::path(
    post,
    path = "/api/v1/categories",
    tag = "categories",
    request_body = openapi::NewCategory,
    responses(
        (status = 201, description = "Category created"),
        (status = 400, description = "Validation failure (name shorter than 2 characters)", body = openapi::ApiError),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn create(
    user: Option<Identity>,
    body: web::Json<NewCategory>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(unauthorized) = require_login(&user) {
        return Ok(unauthorized);
    }
    if let Err(errors) = body.validate() {
        return Ok(ApiError::json(errors.to_string()));
    }

    categories_service::create_category(&state.categories, &body.name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())
}

/// Rename a category. Requires login.
#[utoipa::path(
    put,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    params(("id" = i32, Path, description = "Category id")),
    request_body = openapi::NewCategory,
    responses(
        (status = 200, description = "Category updated"),
        (status = 400, description = "Validation failure", body = openapi::ApiError),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn update(
    user: Option<Identity>,
    path: web::Path<i32>,
    body: web::Json<NewCategory>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(unauthorized) = require_login(&user) {
        return Ok(unauthorized);
    }
    if let Err(errors) = body.validate() {
        return Ok(ApiError::json(errors.to_string()));
    }

    categories_service::update_category(&state.categories, path.into_inner(), &body.name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a category. Requires login.
#[utoipa::path(
    delete,
    path = "/api/v1/categories/{id}",
    tag = "categories",
    params(("id" = i32, Path, description = "Category id")),
    responses(
        (status = 204, description = "Category deleted"),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn delete(
    user: Option<Identity>,
    path: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(unauthorized) = require_login(&user) {
        return Ok(unauthorized);
    }

    categories_service::delete_category(&state.categories, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
