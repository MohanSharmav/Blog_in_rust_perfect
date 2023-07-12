use crate::model::database::{Categories, Posts, PostsCategories};
use sqlx::{Pool, Postgres};

pub async fn get_all_categories_database(
    db: &Pool<Postgres>,
) -> Result<Vec<Categories>, anyhow::Error> {
    let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories")
        .fetch_all(db)
        .await?;

    Ok(all_categories)
}

pub async fn create_new_category_database(
    db: &Pool<Postgres>,
    name: &String,
    id: &i32,
) -> Result<(), anyhow::Error> {
    sqlx::query("insert into categories(id,name) values ($1,$2) ")
        .bind(id)
        .bind(name)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_category_database(
    db: &Pool<Postgres>,
    to_delete_category: &str,
) -> Result<(), anyhow::Error> {
    let to_delete_category: i32 = to_delete_category.parse::<i32>()?;
    // // sqlx::query("delete from posts where category_id=$1")
    //     .bind(to_delete_category)
    //     .execute(db)
    //     .await?;

    sqlx::query("delete from categories where id=$1")
        .bind(to_delete_category)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn update_category_database(
    name: &String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    // let category_id = category_id.parse::<i32>()?;
    sqlx::query("update categories set name=$1 where id=$2")
        .bind(name)
         .bind(category_id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn category_pagination_controller_database_function(
    category_id: String,
    db: &Pool<Postgres>,
) -> Result<Vec<PostsCategories>, anyhow::Error> {
    println!("😀");
     let category_id = category_id.parse::<i32>()?;
    // "select id,title, description,category_id from posts where category_id=$1",
    // "select id,title, description from posts",
//    let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories")
    let category_posts = sqlx::query_as::<_, PostsCategories>(
        "select posts.title,posts.id,posts.description,categories.name from posts,categories_posts,categories where categories_posts.post_id=posts.id and categories.id=categories_posts.category_id and categories_posts.category_id=$1"
    )
     .bind(category_id)
    .fetch_all(db)
    .await?;

    Ok(category_posts)
}
