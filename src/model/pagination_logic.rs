use crate::controller::common_controller::set_posts_per_page;
use crate::model::database::Posts;
use sqlx::{Pool, Postgres};

pub async fn select_specific_pages_post(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let posts_per_page = set_posts_per_page().await;
    let perfect_posts = sqlx::query_as::<_, Posts>(
        "select * from posts Order By id Asc limit $1 OFFSET ($2-1)*$1 ",
    )
    //select * from posts Order By id Asc limit 3 OFFSET (1-1)*3 ;
    .bind(posts_per_page)
    .bind(start_page)
    .fetch_all(db)
    .await?;
    Ok(perfect_posts)
}
