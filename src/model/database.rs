use serde::Deserialize;
use serde::Serialize;
use sqlx::{Pool, Postgres};

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
pub struct CreateNewCategory {
    pub(crate) name: String,
}
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct GetId {
    pub id: i32,
}

pub async fn select_posts(db: &Pool<Postgres>) -> Result<Vec<Posts>, anyhow::Error> {
    let postsing = sqlx::query_as::<_, Posts>("select id, title, description from posts")
        .fetch_all(db)
        .await?;

    Ok(postsing)
}
