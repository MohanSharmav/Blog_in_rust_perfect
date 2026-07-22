//! The application core: pagination rules, credentials, the `PasswordCipher`
//! port, and the post/category/auth use-case services. No HTTP, template, or
//! SQL concerns — those belong to `blog-server`'s adapters and to
//! `blog-storage` respectively. This crate depends on `blog-storage` only for
//! its domain types (`Post`, `Category`, ...) and repository port traits; it
//! never depends on a concrete Postgres/SQLite implementation.

pub mod auth_service;
pub mod categories_service;
pub mod credentials;
pub mod error;
pub mod pagination;
pub mod ports;
pub mod posts_service;

#[cfg(test)]
mod test_fakes;

pub use error::CoreError;
