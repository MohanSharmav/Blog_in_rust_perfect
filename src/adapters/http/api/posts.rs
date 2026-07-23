#[allow(unused_imports)]
use crate::adapters::http::api::openapi;
use crate::adapters::http::api::{require_login, ApiError, PageResponse};
use crate::adapters::http::state::AppState;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use blog_core::posts_service;
use blog_storage::domain::post::NewPost;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
}

/// List posts (paginated, 3 per page).
#[utoipa::path(
    get,
    path = "/api/v1/posts",
    tag = "posts",
    params(("page" = Option<i64>, Query, description = "1-indexed page number, default 1")),
    responses(
        (status = 200, body = openapi::PageOfPosts),
        (status = 404, description = "Page out of range", body = openapi::ApiError),
    )
)]
pub async fn list(
    query: web::Query<PageQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = query.page.unwrap_or(1);
    match posts_service::list_posts(&state.posts, page)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(listing) => Ok(HttpResponse::Ok().json(PageResponse::from(listing))),
        None => Ok(HttpResponse::NotFound().json(ApiError {
            error: "page out of range".to_string(),
        })),
    }
}

/// Get a single post by id.
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post id")),
    responses(
        (status = 200, body = openapi::Post),
        (status = 404, description = "Post not found", body = openapi::ApiError),
    )
)]
pub async fn get(
    path: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = posts_service::get_post(&state.posts, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match posts.into_iter().next() {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(ApiError {
            error: "post not found".to_string(),
        })),
    }
}

/// Create a post. Requires login.
#[utoipa::path(
    post,
    path = "/api/v1/posts",
    tag = "posts",
    request_body = openapi::NewPost,
    responses(
        (status = 201, description = "Post created"),
        (status = 400, description = "Validation failure (empty title/description)", body = openapi::ApiError),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn create(
    user: Option<Identity>,
    body: web::Json<NewPost>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(unauthorized) = require_login(&user) {
        return Ok(unauthorized);
    }
    if let Err(errors) = body.validate() {
        return Ok(ApiError::json(errors.to_string()));
    }

    posts_service::create_post(&state.posts, &body)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())
}

/// Replace a post. Requires login.
#[utoipa::path(
    put,
    path = "/api/v1/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post id")),
    request_body = openapi::NewPost,
    responses(
        (status = 200, description = "Post updated"),
        (status = 400, description = "Validation failure", body = openapi::ApiError),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn update(
    user: Option<Identity>,
    path: web::Path<i32>,
    body: web::Json<NewPost>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(unauthorized) = require_login(&user) {
        return Ok(unauthorized);
    }
    if let Err(errors) = body.validate() {
        return Ok(ApiError::json(errors.to_string()));
    }

    posts_service::update_post(&state.posts, path.into_inner(), &body)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a post. Requires login.
#[utoipa::path(
    delete,
    path = "/api/v1/posts/{id}",
    tag = "posts",
    params(("id" = i32, Path, description = "Post id")),
    responses(
        (status = 204, description = "Post deleted"),
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

    posts_service::delete_post(&state.posts, path.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
