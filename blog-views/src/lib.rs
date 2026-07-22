//! Bundles the blog's Handlebars templates and static assets (CSS/JS/images)
//! so `blog-server` doesn't need to know their filesystem layout — just
//! [`register`] and [`root`].

use handlebars::Handlebars;

pub mod error;
pub use error::ViewsError;

/// Absolute path to this crate's bundled `templates/` directory as it was
/// laid out *at compile time*. Correct for `cargo run`/`cargo build` (the
/// source tree doesn't move), but a compiled binary copied somewhere else —
/// a Docker image's runtime stage, an installed `/usr/local/bin` binary —
/// needs [`root`]'s `BLOG_VIEWS_ROOT` override instead, since this constant
/// can't know where the binary will actually live.
pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");

/// The templates/assets root to use at runtime: the `BLOG_VIEWS_ROOT`
/// environment variable if set, otherwise the compile-time [`ROOT`].
pub fn root() -> String {
    std::env::var("BLOG_VIEWS_ROOT").unwrap_or_else(|_| ROOT.to_string())
}

/// Registers every `.html` and `.hbs` file under `{root()}/html` into `handlebars`.
pub fn register(handlebars: &mut Handlebars) -> error::Result<()> {
    let html_dir = format!("{}/html", root());
    handlebars
        .register_templates_directory(".html", &html_dir)
        .map_err(Box::new)?;
    handlebars
        .register_templates_directory(".hbs", &html_dir)
        .map_err(Box::new)?;
    Ok(())
}
