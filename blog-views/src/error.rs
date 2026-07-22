//! This crate's own error type, in place of an opaque `anyhow::Error`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ViewsError {
    // Boxed: `TemplateError` is large enough that clippy's `result_large_err`
    // flags an unboxed `Result<T, ViewsError>` as bloating every `Ok` path too.
    #[error("failed to register templates: {0}")]
    Template(#[from] Box<handlebars::TemplateError>),
}

pub type Result<T> = std::result::Result<T, ViewsError>;
