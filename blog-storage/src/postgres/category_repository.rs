use crate::domain::category::Category;
use crate::error::Result;
use crate::ports::CategoryRepository;
use sqlx::{Pool, Postgres};

/// Postgres-backed adapter implementing the [`CategoryRepository`] port.
#[derive(Clone)]
pub struct PgCategoryRepository {
    pool: Pool<Postgres>,
}

impl PgCategoryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl CategoryRepository for PgCategoryRepository {
    async fn all(&self) -> Result<Vec<Category>> {
        Ok(
            sqlx::query_as::<_, Category>("select name,id from categories")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn all_except(&self, id: i32) -> Result<Vec<Category>> {
        Ok(
            sqlx::query_as::<_, Category>(" select * from categories where Not id=$1")
                .bind(id)
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn page(&self, page_number: i32, per_page: i32) -> Result<Vec<Category>> {
        Ok(sqlx::query_as::<_, Category>(
            "select name,id  from categories Order By id Asc limit $2 offset ($1-1)*$2",
        )
        .bind(page_number)
        .bind(per_page)
        .fetch_all(&self.pool)
        .await?)
    }

    async fn find(&self, id: i32) -> Result<Vec<Category>> {
        Ok(
            sqlx::query_as::<_, Category>("select name,id from categories where id=$1")
                .bind(id)
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn count(&self) -> Result<i64> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM categories")
            .fetch_one(&self.pool)
            .await?)
    }

    async fn create(&self, name: &str) -> Result<()> {
        sqlx::query("insert into categories(name) values ($1) ")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update(&self, id: i32, name: &str) -> Result<()> {
        sqlx::query("update categories set name=$1 where id=$2")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("delete from categories_posts where category_id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("delete from categories where id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
