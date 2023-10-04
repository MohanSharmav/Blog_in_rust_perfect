use crate::controllers::admin::posts_controller::Post;
use crate::model::categories::{GetCategoryId, GetId};
use crate::SET_POSTS_PER_PAGE;
use sqlx::{Pool, Postgres, Row};

pub async fn delete_post_db(post_id: String, db: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let to_delete = post_id.parse::<i32>()?;
    // remove post id from 3rd database categories_posts
    sqlx::query("delete from categories_posts where post_id=$1")
        .bind(to_delete)
        .execute(db)
        .await?;
    // delete from posts main table
    sqlx::query("delete from posts where id=$1")
        .bind(to_delete)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn update_post_db(
    title: &str,
    description: &str,
    post_id: i32,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(post_id)
        .execute(db)
        .await?;

    sqlx::query("update categories_posts set category_id=$2 where post_id=$1")
        .bind(post_id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn create_post(
    title: &str,
    description: &str,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    // use this query to get the id of the newly created post
    // this is because id is generated dynamically
    // you need post_id to link it with the category_id in 3rd table categories_posts
    let GetId { id } = sqlx::query_as::<_, GetId>(
        "insert into posts(title,description) values($1,$2) returning id",
    )
    .bind(title)
    .bind(description)
    .fetch_one(db)
    .await?;
    // insert the dynamically generated id and category_id to 3rd table and link

    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_one(db)
        .await?;

    Ok(())
}

pub async fn create_post_without_category(
    title: &str,
    description: &str,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    // do not touch 3rd table because post has no categroy
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
    post_id: i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
        .bind(title)
        .bind(description)
        .bind(post_id)
        .execute(db)
        .await?;
    // delete id from 3rd table and remove the link between post and category
    // category -> no category update so
    sqlx::query("delete from categories_posts where post_id=$1")
        .bind(post_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn category_id_from_post_id(
    post_id: i32,
    db: &Pool<Postgres>,
) -> Result<i32, anyhow::Error> {
    Ok(sqlx::query_as::<_, GetCategoryId>(
        "select category_id from categories_posts where post_id=$1",
    )
    .bind(post_id)
    .fetch_optional(db)
    .await?
    .map(|inner| inner.category_id)
    .unwrap_or_default())
}

pub async fn specific_page_posts(
    cur_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Post>, anyhow::Error> {
    sqlx::query_as::<_, Post>("select * from posts Order By id Asc limit $1 OFFSET ($2-1)*$1 ")
        .bind(*SET_POSTS_PER_PAGE)
        .bind(cur_page)
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub async fn find_post(post_id: i32, db: &Pool<Postgres>) -> Result<Vec<Post>, anyhow::Error> {
    sqlx::query_as::<_, Post>("select id, title, description from posts  WHERE id=$1")
        .bind(post_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub async fn update_post_from_no_category(
    title: &str,
    description: &str,
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
    // no category -> category
    // so insert category id to 3rd table
    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .fetch_all(db)
        .await?;

    Ok(())
}

pub async fn number_posts_count(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_one(db)
        .await
        .and_then(|row| row.try_get("count"))
        .map_err(actix_web::error::ErrorInternalServerError)
}
