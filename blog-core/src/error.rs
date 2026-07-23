//! This crate's own error type, in place of an opaque `anyhow::Error` — the
//! use-case services below only ever fail because the storage layer did, so
//! this just names that instead of hiding it behind `anyhow`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Storage(#[from] blog_storage::StorageError),
}

pub type Result<T> = std::result::Result<T, CoreError>;
