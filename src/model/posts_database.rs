use sqlx::{Pool, Postgres};

pub async fn delete_post_database(
    to_delete: String,
    db: &Pool<Postgres>,
) -> Result<(), anyhow::Error> {
    let to_delete = to_delete.parse::<i32>()?;
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
    sqlx::query("update posts set title=$1 ,description=$2, category_id=$3 where id=$4")
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}
