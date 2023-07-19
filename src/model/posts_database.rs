use std::process::id;
use hmac::digest::impl_oid_carrier;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use crate::model::database::GetId;

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
    id: &&i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn create_post_database(
    title: String,
    description: String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    // let id = id as i32;
    //200
    // sqlx::query("insert into posts values($1,$2,$3)")
// let post_id=sqlx::query("insert into posts(title,description) values($1,$2) returning id")
   let post_id= sqlx::query_as::<_, GetId>("insert into posts(title,description) values($1,$2) returning id")
    .bind(title)
        .bind(description)
       .fetch_all(db)
        .await?;
    //get 200
// let post_id =post_id.into().unwrap_or_default();
    let x: &GetId =&post_id[0];
    let GetId{id}=x;
//     let GetId{id} =post_id;
//     let post_id =<PgQueryResult as Into<T>>::into(post_id).unwrap_or_default();

    //get id

println!("----------------------------------------------------here--------------------------------");
    //send 200
    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}
