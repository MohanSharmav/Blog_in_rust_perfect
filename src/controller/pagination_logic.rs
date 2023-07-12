use crate::model::database::Posts;
use sqlx::{Pool, Postgres};
use crate::controller::common_controller::set_posts_per_page;

pub async fn select_specific_pages_post(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let mut new_start_page = start_page;
    if start_page > 1 {
        new_start_page += 2
    }

    let posts_per_page=set_posts_per_page().await;
    //
    // let perfect_posts = sqlx::query_as::<_, Posts>("select * from posts limit  $1 offset $2")
    //     .bind(new_start_page + 3)
    //     .bind(new_start_page)
    //     .fetch_all(db)
    //     .await?;
    let perfect_posts = sqlx::query_as::<_, Posts>("select * from posts limit $1 OFFSET ($2-1)*$1")
        .bind(posts_per_page)
        .bind(start_page)
        // .bind(new_start_page)
        .fetch_all(db)
        .await?;
    Ok(perfect_posts)
}
