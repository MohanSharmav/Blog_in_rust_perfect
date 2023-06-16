use crate::model::database::Posts;
use sqlx::{Pool, Postgres, Row};

pub async fn query_single_post(
    titles: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<String>, anyhow::Error> {
    let mut single_post = Vec::new();
    let rows = sqlx::query("SELECT title,description FROM posts WHERE id=$1")
        .bind(titles)
        .fetch_all(db)
        .await?;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let single_post_string = title + " " + &*description;
        single_post.push(single_post_string);
    }
    Ok(single_post)
}
pub async fn query_single_post_in_struct(
    titles: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let single_post = sqlx::query_as::<_, Posts>(
        "select id, title, description, category_id from posts  WHERE id=$1",
    )
    .bind(titles)
    .fetch_all(db)
    .await?;
    Ok(single_post)
}
