use serde::{Deserialize, Serialize};
use validator::Validate;

/// A post category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

/// The data required to create or rename a category, as submitted by a client.
#[derive(Debug, Clone, PartialEq, Deserialize, Validate)]
pub struct NewCategory {
    #[validate(length(
        min = 2,
        message = "category name cannot be empty and minimum should have 2 characters"
    ))]
    pub name: String,
}
