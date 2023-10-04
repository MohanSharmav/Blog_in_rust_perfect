use crate::controllers::admin::posts_controller::PostsCategory;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use std::result;

pub async fn all_categories(db: &Pool<Postgres>) -> Result<Vec<Category>, anyhow::Error> {
    sqlx::query_as::<_, Category>("select name,id from categories")
        .fetch_all(db)
        .await
        .map_err(anyhow::Error::new)
}

pub async fn create_new_category_db(
    db: &Pool<Postgres>,
    category_name: &String,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into categories(name) values ($1)")
        .bind(category_name)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_category_db(
    db: &Pool<Postgres>,
    category_id: String,
) -> Result<(), anyhow::Error> {
    let category_id: i32 = category_id.parse::<i32>()?;
    // delete id from 3rd table [categories_posts]
    // to avoid primary key constraints
    sqlx::query("delete from categories_posts where category_id=$1")
        .bind(category_id)
        .execute(db)
        .await?;
    // remove id from categories main table
    sqlx::query("delete from categories where id=$1")
        .bind(category_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn update_category_db(
    new_category_name: &String,
    category_id: i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(new_category_name)
        .bind(category_id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn category_based_posts_db(
    category_id: String,
    db: &Pool<Postgres>,
    par: i32,
    posts_per_page: i64,
) -> Result<Vec<PostsCategory>, anyhow::Error> {
    sqlx::query_as::<_, PostsCategory>(
        "select posts.title,posts.id,posts.description,categories.name  from posts,categories_posts,categories  where categories_posts.post_id=posts.id and categories.id=categories_posts.category_id and categories_posts.category_id=$1 Order By posts.id Asc  limit $3 offset($2-1)*$3"
    )
        .bind(category_id.parse::<i32>()?)
        .bind(par)
        .bind(posts_per_page)
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub async fn get_all_categories_db(
    db: &Pool<Postgres>,
    cur_page: i32,
    posts_per_page: i64,
) -> Result<Vec<Category>, anyhow::Error> {
    sqlx::query_as::<_, Category>(
        "select name,id  from categories Order By id Asc limit $2 offset ($1-1)*$2",
    )
    .bind(cur_page)
    .bind(posts_per_page)
    .fetch_all(db)
    .await
    .map_err(Into::into)
}

pub async fn find_categories(id: i32, db: &Pool<Postgres>) -> Result<Vec<Category>, anyhow::Error> {
    sqlx::query_as::<_, Category>("select name,id from categories where id=$1")
        .bind(id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub async fn individual_category_posts_count(
    category_input: &str,
    db: &Pool<Postgres>,
) -> Result<i64, anyhow::Error> {
    sqlx::query("SELECT COUNT(*) FROM categories_posts where category_id=$1")
        .bind(category_input.parse::<i32>()?)
        .fetch_one(db)
        .await
        .and_then(|inner| inner.try_get("count"))
        .map_err(Into::into)
}

pub async fn all_categories_exclusive(
    db: &Pool<Postgres>,
    category_id: i32,
) -> Result<Vec<Category>, anyhow::Error> {
    // Get all categories name expect the given category_id
    sqlx::query_as::<_, Category>(" select * from categories where Not id=$1")
        .bind(category_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Category {
    pub(crate) id: i32,
    pub(crate) name: String,
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
pub struct GetId {
    pub id: i32,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct GetCategoryId {
    pub category_id: i32,
}

pub async fn categories_count(db: &Pool<Postgres>) -> result::Result<i64, actix_web::error::Error> {
    sqlx::query("SELECT COUNT(*) FROM categories")
        .fetch_one(db)
        .await
        .and_then(|row| row.try_get("count"))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))
}
