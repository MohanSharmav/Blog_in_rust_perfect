use crate::model::structs::{Categories, PostsCategories};
use anyhow::anyhow;
use sqlx::{Pool, Postgres, Row};

pub async fn all_categories_db(
    db: &Pool<Postgres>,
) -> Result<Vec<Categories>, anyhow::Error> {
    let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories")
        .fetch_all(db)
        .await?;

    Ok(all_categories)
}

pub async fn create_new_category_db(
    db: &Pool<Postgres>,
    name: &String,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into categories(name) values ($1) ")
        .bind(name)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_category_db(
    db: &Pool<Postgres>,
    to_delete_category: &str,
) -> Result<(), anyhow::Error> {
    let to_delete_category: i32 = to_delete_category.parse::<i32>()?;

    sqlx::query("delete from categories_posts where category_id=$1")
        .bind(to_delete_category)
        .execute(db)
        .await?;

    sqlx::query("delete from categories where id=$1")
        .bind(to_delete_category)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn update_category_db(
    name: &String,
    category_id: i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(name)
        .bind(category_id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn category_db(
    category_id: String,
    db: &Pool<Postgres>,
    par: i32,
    posts_per_page: i64,
) -> Result<Vec<PostsCategories>, anyhow::Error> {
    let category_id = category_id.parse::<i32>()?;
    let category_posts = sqlx::query_as::<_, PostsCategories>(
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
    parii: i32,
    posts_per_page_constant: i32,
) -> Result<Vec<Categories>, anyhow::Error> {
    let all_categories = sqlx::query_as::<_, Categories>(
        "select name,id  from categories Order By id Asc limit $2 offset ($1-1)*$2",
    )
    .bind(parii)
    .bind(posts_per_page_constant)
    .fetch_all(db)
    .await?;

    Ok(all_categories)
}

pub async fn get_specific_category_posts(
    id: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Categories>, anyhow::Error> {
    let all_categories =
        sqlx::query_as::<_, Categories>("select name,id from categories where id=$1")
            .bind(id)
            .fetch_all(db)
            .await?;

    Ok(all_categories)
}

pub async fn category_pagination_logic(
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

    let a = counting_final.get(0).ok_or(anyhow!("{}", "error"))?;
    let c = a
        .as_ref()
        .map(|i| *i)
        .map_err(|_e| anyhow::Error::msg("failed"))?;

    Ok(c)
}
