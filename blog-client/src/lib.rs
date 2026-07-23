//! A typed async client for the blog-server JSON API (`/api/v1/...`).

mod client;
mod error;
mod types;

pub use client::BlogClient;
pub use error::ClientError;
pub use types::{Category, Credentials, NewCategory, NewPost, Page, Post};
