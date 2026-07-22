//! Storage layer for the blog: domain types, repository ports, and one
//! implementation per supported database, chosen at compile time via Cargo
//! features rather than a runtime enum. This crate itself compiles fine with
//! zero, one, or both backend features enabled (so its own test suite can
//! exercise both); it's the final binary's job — see `blog`'s `main.rs` — to
//! require exactly one.

pub mod domain;
pub mod error;
pub mod ports;

pub use error::StorageError;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;
