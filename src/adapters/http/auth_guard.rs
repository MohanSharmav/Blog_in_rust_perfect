use actix_identity::Identity;
use actix_web::{http, HttpResponse};

/// Returns a redirect-to-home response when no user is logged in, so admin
/// handlers can early-return with `if let Some(redirect) = require_login(&user) { return Ok(redirect); }`.
pub fn require_login(user: &Option<Identity>) -> Option<HttpResponse> {
    if user.is_some() {
        return None;
    }

    Some(
        HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""),
    )
}
