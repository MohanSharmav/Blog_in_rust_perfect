//! This crate's own error type, in place of an opaque `anyhow::Error` —
//! callers can match on `StorageError` if they ever need to (e.g. treat a
//! constraint violation differently from a connection failure), and the
//! `#[error(...)]` messages are what ends up in logs when one bubbles up
//! through `blog-core`/`blog-server`'s `anyhow` composition root.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

pub type Result<T> = std::result::Result<T, StorageError>;
