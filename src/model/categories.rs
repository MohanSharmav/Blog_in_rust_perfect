use crate::controllers::admin::posts_controller::PostsCategory;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use std::result;

pub async fn all_categories_db(db: &Pool<Postgres>) -> Result<Vec<Category>, anyhow::Error> {
    let all_categories = sqlx::query_as::<_, Category>("select name,id from categories")
        .fetch_all(db)
        .await?;

    Ok(all_categories)
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
    category_id: &str,
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
    let category_id = category_id.parse::<i32>()?;
    let category_posts = sqlx::query_as::<_, PostsCategory>(
        "select posts.title,posts.id,posts.description,categories.name  from posts,categories_posts,categories  where categories_posts.post_id=posts.id and categories.id=categories_posts.category_id and categories_posts.category_id=$1 Order By posts.id Asc  limit $3 offset($2-1)*$3"
    )
        .bind(category_id)
        .bind(par)
        .bind(posts_per_page)
        .fetch_all(db)
        .await?;

    Ok(category_posts)
}

pub async fn get_all_categories_db(
    db: &Pool<Postgres>,
    current_page: i32,
    posts_per_page_constant: i64,
) -> Result<Vec<Category>, anyhow::Error> {
    let all_categories = sqlx::query_as::<_, Category>(
        "select name,id  from categories Order By id Asc limit $2 offset ($1-1)*$2",
    )
    .bind(current_page)
    .bind(posts_per_page_constant)
    .fetch_all(db)
    .await?;

    Ok(all_categories)
}

pub async fn get_specific_category_posts(
    id: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Category>, anyhow::Error> {
    let all_categories =
        sqlx::query_as::<_, Category>("select name,id from categories where id=$1")
            .bind(id)
            .fetch_all(db)
            .await?;

    Ok(all_categories)
}

pub async fn individual_category_posts_count(
    category_input: &str,
    db: &Pool<Postgres>,
) -> Result<i64, anyhow::Error> {
    let category_id = category_input.parse::<i32>()?;
    let rows = sqlx::query("SELECT COUNT(*) FROM categories_posts where category_id=$1")
        .bind(category_id)
        .fetch_all(db)
        .await?;

    let counting_final: Vec<Result<i64, anyhow::Error>> = rows
        .into_iter()
        .map(|row| {
            let counting_final: i64 = row
                .try_get("count")
                .map_err(|_e| anyhow::Error::msg("failed"))?;
            Ok::<i64, anyhow::Error>(counting_final)
        })
        .collect();

    let before_remove_error = counting_final.get(0).ok_or(anyhow!("{}", "error"))?;
    let exact_value = before_remove_error
        .as_ref()
        .map(|i| *i)
        .map_err(|_e| anyhow::Error::msg("failed"))?;

    Ok(exact_value)
}

pub async fn all_categories_exclusive(
    db: &Pool<Postgres>,
    category_id: i32,
) -> Result<Vec<Category>, anyhow::Error> {
    // Get all categories name expect the given category_id
    let all_categories = sqlx::query_as::<_, Category>(" select * from categories where Not id=$1")
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(all_categories)
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

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct GetCategoryId {
    pub category_id: i32,
}

pub async fn categories_count(db: &Pool<Postgres>) -> result::Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM categories")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<result::Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let before_remove_error = counting_final
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let exact_value = before_remove_error
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*exact_value)
}
