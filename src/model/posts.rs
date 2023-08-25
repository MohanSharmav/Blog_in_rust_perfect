use crate::controllers::admin::posts_controller::Posts;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::model::categories::{GetCategoryId, GetId};
use sqlx::{Pool, Postgres, Row};

pub async fn delete_post_db(to_delete: String, db: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let to_delete = to_delete.parse::<i32>()?;
    sqlx::query("delete from categories_posts where post_id=$1")
        .bind(to_delete)
        .execute(db)
        .await?;

    sqlx::query("delete from posts where id=$1")
        .bind(to_delete)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn update_post_db(
    title: &String,
    description: &String,
    id: i32,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(db)
        .await?;

    sqlx::query("update categories_posts set category_id=$2 where post_id=$1")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn create_post(
    title: String,
    description: String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    let post_id = sqlx::query_as::<_, GetId>(
        "insert into posts(title,description) values($1,$2) returning id",
    )
    .bind(title)
    .bind(description)
    .fetch_all(db)
    .await?;

    let post_id: &GetId = &post_id[0];
    let GetId { id } = post_id;

    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn create_post_without_category(
    title: String,
    description: String,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into posts(title,description) values ($1,$2)")
        .bind(title)
        .bind(description)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn update_post_without_category(
    title: String,
    description: String,
    id: i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(db)
        .await?;

    sqlx::query("delete from categories_posts where post_id=$1")
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn category_id_from_post_id(
    postid: i32,
    db: &Pool<Postgres>,
) -> Result<i32, anyhow::Error> {
    let category_id_vec = sqlx::query_as::<_, GetCategoryId>(
        "select category_id from categories_posts where post_id=$1",
    )
    .bind(postid)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    let category_id = category_id_vec
        .get(0)
        .map(|i| i.category_id)
        .unwrap_or_default();

    Ok(category_id)
}

pub async fn specific_page_posts(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let start_page = start_page;
    let posts_per_page = SET_POSTS_PER_PAGE;
    let perfect_posts = sqlx::query_as::<_, Posts>(
        "select * from posts Order By id Asc limit $1 OFFSET ($2-1)*$1 ",
    )
    .bind(posts_per_page)
    .bind(start_page)
    .fetch_all(db)
    .await?;

    Ok(perfect_posts)
}

pub async fn query_single_post(
    titles: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<String>, anyhow::Error> {
    let rows = sqlx::query("SELECT title,description FROM posts WHERE id=$1")
        .bind(titles)
        .fetch_all(db)
        .await?;

    let single_post = rows
        .iter()
        .map(|row| {
            let title: String = row.get("title");
            let description: String = row.get("description");
            title + " " + &description
        })
        .collect();

    Ok(single_post)
}

pub async fn single_post_db(titles: i32, db: &Pool<Postgres>) -> Result<Vec<Posts>, anyhow::Error> {
    let single_post =
        sqlx::query_as::<_, Posts>("select id, title, description from posts  WHERE id=$1")
            .bind(titles)
            .fetch_all(db)
            .await?;
    Ok(single_post)
}

pub async fn update_post_from_no_category(
    title: &String,
    description: &String,
    category_id: &i32,
    id: i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(db)
        .await?;

    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}
