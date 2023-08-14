use crate::model::structs::GetId;
use core::option::Option;
use sqlx::{Pool, Postgres};

pub async fn delete_post_database(
    to_delete: String,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
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

pub async fn update_post_database(
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

pub async fn create_post_database(
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
    let x: &GetId = &post_id[0];
    let GetId { id } = x;
    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn create_post_without_category_database(
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

pub async fn update_post_without_category_database(
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

pub async fn get_category_id_from_post_id(
    postid: i32,
    db: &Pool<Postgres>,
) -> Result<Option<i32>, anyhow::Error> {
    //todo
    let category_id =
        sqlx::query_as::<_, GetId>("select category_id from categories_posts where post_id=$1")
            .bind(postid)
            .fetch_all(db)
            .await?;
    let x: &GetId = &category_id[0];
    let GetId { id } = x;
    // Ok(Some(*id))
    Ok::<std::option::Option<i32>, anyhow::Error>(Some(*id))
}
