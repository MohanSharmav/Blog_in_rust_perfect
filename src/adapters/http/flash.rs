use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

/// Renders all pending flash messages into a single newline-separated HTML string.
pub fn render_flash_messages(
    flash_message: &IncomingFlashMessages,
) -> Result<String, actix_web::Error> {
    let mut html = String::new();
    for message in flash_message.iter() {
        writeln!(html, "{}", message.content())
            .map_err(actix_web::error::ErrorInternalServerError)?;
    }
    Ok(html)
}
