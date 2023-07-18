use crate::model::database::{Categories, PostsCategories};
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

pub async fn update_category_database(
    name: &String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
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
    par: i32,
) -> Result<Vec<PostsCategories>, anyhow::Error> {
    let category_id = category_id.parse::<i32>()?;
    let category_posts = sqlx::query_as::<_, PostsCategories>(
        "select posts.title,posts.id,posts.description,categories.name from posts,categories_posts,categories where categories_posts.post_id=posts.id and categories.id=categories_posts.category_id and categories_posts.category_id=$1 limit 3 offset($2-1)*3"
    )
     .bind(category_id)
        .bind(par)
    .fetch_all(db)
    .await?;

    Ok(category_posts)
}


//
// pub async fn get_all_categories_database_with_pagination_display(
//     db: &Pool<Postgres>,
//     parii: i32,
// ) -> Result<Vec<Categories>, anyhow::Error> {
//     let all_categories = sqlx::query_as::<_, Categories>("select name,id from categories limit 3 offset ($1-1)*3")
//         .fetch_all(db)
//         .bind(parii)
//         .await?;
//
//     Ok(all_categories)
// }
