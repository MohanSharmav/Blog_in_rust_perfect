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
// posts.title posts.id,posts.description,categories.name
#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct PostsCategories {
    pub title: String,
    pub id: i32,
    pub description: String,
    pub name: String,
}

pub async fn select_posts(db: &Pool<Postgres>) -> Result<Vec<Posts>, anyhow::Error> {
    let postsing =
        sqlx::query_as::<_, Posts>("select id, title, description from posts")
            .fetch_all(db)
            .await?;

    Ok(postsing)
}
