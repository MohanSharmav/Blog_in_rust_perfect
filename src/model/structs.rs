use argon2::PasswordHash;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use validator::Validate;

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Categories {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Posts {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct PostsCategories {
    pub title: String,
    pub id: i32,
    pub description: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct CreatePost {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub category_id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow, Validate)]
pub struct CreateNewPost {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "description cannot be empty"))]
    pub description: String,
    pub category_id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct CreateNewPostWithoutCategory {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct CreateNewPostWithNullCategory {
    pub category_id: i32,
}
#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow, Validate)]
pub struct CreateNewCategory {
    #[validate(length(
        min = 2,
        message = "category name cannot be empty and minimum should have 2 characters"
    ))]
    pub(crate) name: String,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct GetId {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LoginCheck {
    pub(crate) value: i64,
}

impl<'r> FromRow<'r, PgRow> for LoginCheck {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let name = row.try_get("count")?;
        Ok(LoginCheck { value: name })
    }
}
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct GetCategoryId {
    pub category_id: i32,
}
#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Password {
    pub password: String,
}
