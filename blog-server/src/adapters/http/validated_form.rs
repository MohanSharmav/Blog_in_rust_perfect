use actix_web::{http, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use validator::Validate;

/// Validates `form`; on failure, sends a flash error and returns a redirect
/// to `redirect_to` for the caller to return immediately.
pub fn validate_or_redirect<T: Validate>(form: &T, redirect_to: &str) -> Option<HttpResponse> {
    if let Err(errors) = form.validate() {
        FlashMessage::error(errors.to_string()).send();
        return Some(
            HttpResponse::SeeOther()
                .insert_header((http::header::LOCATION, redirect_to.to_owned()))
                .finish(),
        );
    }
    None
}
