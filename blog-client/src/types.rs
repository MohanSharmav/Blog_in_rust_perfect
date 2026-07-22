//! Wire types for the blog-server JSON API (`/api/v1/...`). These mirror the
//! server's `domain` types structurally but are defined independently here —
//! the JSON payload is the actual contract between client and server, not a
//! shared Rust type.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: String,
    /// `0` means "no category".
    pub category_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// A page of items, as returned by the paginated list endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub total_pages: usize,
    pub total_items: i64,
}
