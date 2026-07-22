use serde::{Deserialize, Serialize};
use validator::Validate;

/// A blog post, independent of any category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
}

/// A post joined with the name of the category it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostWithCategory {
    pub title: String,
    pub id: i32,
    pub description: String,
    pub name: String,
}

/// The data required to create or update a post, as submitted by a client.
#[derive(Debug, Clone, PartialEq, Deserialize, Validate)]
pub struct NewPost {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "description cannot be empty"))]
    pub description: String,
    pub category_id: i32,
}
