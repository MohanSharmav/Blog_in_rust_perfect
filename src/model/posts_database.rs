use sqlx::{Pool, Postgres};

pub async fn delete_post_database(
    to_delete: String,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    let to_delete = to_delete.parse::<i32>()?;
    // delete from categories_posts where post_id=5;
    //
    // delete from posts where id=5

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
    id: usize,
    title: String,
    description: String,
    category_id: &i32,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    //categories_posts
    let id = id as i32;
    println!("{:?} {:?}{:?}{:?}", id, category_id, title, description);
    sqlx::query("insert into posts values($1,$2,$3)")
        .bind(id)
        .bind(title)
        .bind(description)
        .execute(db)
        .await?;

    sqlx::query("insert into categories_posts values ($1,$2)")
        .bind(id)
        .bind(category_id)
        .execute(db)
        .await?;

    Ok(())
}
