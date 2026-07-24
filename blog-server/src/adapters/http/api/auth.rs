#[allow(unused_imports)]
use crate::adapters::http::api::openapi;
use crate::adapters::http::api::ApiError;
use crate::adapters::http::state::AppState;
use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use blog_core::auth_service;
use blog_core::credentials::Credentials;
use serde_json::json;

/// Log in.
#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = "auth",
    request_body = openapi::Credentials,
    responses(
        (status = 200, description = "Authenticated; the session cookie is set via `Set-Cookie`", body = openapi::UsernameResponse),
        (status = 401, description = "Wrong password or unknown username", body = openapi::ApiError),
    )
)]
pub async fn login(
    body: web::Json<Credentials>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let authenticated =
        auth_service::authenticate(&state.users, &state.cipher, &body.username, &body.password)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    if !authenticated {
        return Ok(HttpResponse::Unauthorized().json(ApiError {
            error: "invalid username or password".to_string(),
        }));
    }

    Identity::login(&req.extensions(), body.username.clone())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "username": body.username })))
}

/// Log out. A no-op if not logged in.
#[utoipa::path(
    post,
    path = "/api/v1/logout",
    tag = "auth",
    responses((status = 200, description = "Session cleared (or was already empty)")),
)]
pub async fn logout(user: Option<Identity>) -> HttpResponse {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok().finish()
}

/// The currently logged-in username.
#[utoipa::path(
    get,
    path = "/api/v1/me",
    tag = "auth",
    responses(
        (status = 200, body = openapi::UsernameResponse),
        (status = 401, description = "Not logged in", body = openapi::ApiError),
    )
)]
pub async fn me(user: Option<Identity>) -> Result<HttpResponse, actix_web::Error> {
    let Some(user) = user else {
        return Ok(HttpResponse::Unauthorized().json(ApiError {
            error: "authentication required".to_string(),
        }));
    };
    let username = user
        .id()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({ "username": username })))
}

/// Create an account. Does not log the new user in.
#[utoipa::path(
    post,
    path = "/api/v1/register",
    tag = "auth",
    request_body = openapi::Credentials,
    responses((status = 201, description = "Account created")),
)]
pub async fn register(
    body: web::Json<Credentials>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    auth_service::register(&state.users, &state.cipher, &body.username, &body.password)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())
}
