use crate::controllers::admin::posts_controller::Post;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::model::categories::{GetCategoryId, GetId};
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
    title: &String,
    description: &String,
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
    title: String,
    description: String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    // use this query to get the id of the newly created post
    // this is because id is generated dynamically
    // you need post_id to link it with the category_id in 3rd table categories_posts
    let post_id = sqlx::query_as::<_, GetId>(
        "insert into posts(title,description) values($1,$2) returning id",
    )
    .bind(title)
    .bind(description)
    .fetch_all(db)
    .await?;
    // remove id from vector
    let post_id: &GetId = &post_id[0];
    let GetId { id } = post_id;
    // insert the dynamically generated id and category_id to 3rd table and link
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
    let category_id_vec = sqlx::query_as::<_, GetCategoryId>(
        "select category_id from categories_posts where post_id=$1",
    )
    .bind(post_id)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    let category_id = category_id_vec
        .get(0)
        .map(|value| value.category_id)
        .unwrap_or_default();

    Ok(category_id)
}

pub async fn specific_page_posts(
    start_page: i32,
    db: &Pool<Postgres>,
) -> Result<Vec<Post>, anyhow::Error> {
    let start_page = start_page;
    let posts_per_page = SET_POSTS_PER_PAGE;
    let perfect_posts =
        sqlx::query_as::<_, Post>("select * from posts Order By id Asc limit $1 OFFSET ($2-1)*$1 ")
            .bind(posts_per_page)
            .bind(start_page)
            .fetch_all(db)
            .await?;

    Ok(perfect_posts)
}

pub async fn single_post_db(titles: i32, db: &Pool<Postgres>) -> Result<Vec<Post>, anyhow::Error> {
    let single_post =
        sqlx::query_as::<_, Post>("select id, title, description from posts  WHERE id=$1")
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

pub async fn number_posts_count(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<Result<i64, actix_web::Error>> = rows
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
