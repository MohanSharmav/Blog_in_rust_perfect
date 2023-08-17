use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

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
pub struct UpdatePost {
    pub(crate) current_title: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
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

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct CreateNewPost {
    pub title: String,
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
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct CreateNewCategory {
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
