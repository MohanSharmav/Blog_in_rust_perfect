//! JSON API adapter: the same application services used by the HTML admin
//! pages, exposed as a `/api/v1` REST surface for `blog-client`/`blog-cli`.
//! Authentication reuses the existing cookie-session `Identity` middleware —
//! `blog-client` just needs to keep a cookie jar across requests.

pub mod auth;
pub mod categories;
pub mod openapi;
pub mod posts;

use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use blog_core::pagination::Listing;
use serde::Serialize;

/// A uniform error body for every non-2xx API response.
#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn json(message: impl Into<String>) -> HttpResponse {
        HttpResponse::BadRequest().json(ApiError {
            error: message.into(),
        })
    }
}

/// A page of items, shaped for JSON clients (mirrors `domain::pagination::Listing`).
#[derive(Serialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub total_pages: usize,
    pub total_items: i64,
}

impl<T> From<Listing<T>> for PageResponse<T> {
    fn from(listing: Listing<T>) -> Self {
        Self {
            items: listing.items,
            page: listing.page.current,
            total_pages: listing.page.total,
            total_items: listing.total_items,
        }
    }
}

/// Returns a `401` JSON body when no user is logged in.
pub fn require_login(user: &Option<Identity>) -> Option<HttpResponse> {
    if user.is_some() {
        return None;
    }
    Some(HttpResponse::Unauthorized().json(ApiError {
        error: "authentication required".to_string(),
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/login", web::post().to(auth::login))
            .route("/logout", web::post().to(auth::logout))
            .route("/register", web::post().to(auth::register))
            .route("/me", web::get().to(auth::me))
            .route("/posts", web::get().to(posts::list))
            .route("/posts", web::post().to(posts::create))
            .route("/posts/{id}", web::get().to(posts::get))
            .route("/posts/{id}", web::put().to(posts::update))
            .route("/posts/{id}", web::delete().to(posts::delete))
            .route("/categories", web::get().to(categories::list))
            .route("/categories", web::post().to(categories::create))
            .route("/categories/{id}", web::get().to(categories::get))
            .route("/categories/{id}", web::put().to(categories::update))
            .route("/categories/{id}", web::delete().to(categories::delete)),
    );
}
