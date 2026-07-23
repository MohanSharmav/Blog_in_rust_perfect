use crate::domain::post::{NewPost, Post, PostWithCategory};
use crate::error::Result;
use crate::ports::PostRepository;
use sqlx::{Pool, Postgres};

/// Postgres-backed adapter implementing the [`PostRepository`] port.
#[derive(Clone)]
pub struct PgPostRepository {
    pool: Pool<Postgres>,
}

impl PgPostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl PostRepository for PgPostRepository {
    async fn count(&self) -> Result<i64> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM posts")
            .fetch_one(&self.pool)
            .await?)
    }

    async fn count_for_category(&self, category_id: i32) -> Result<i64> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM categories_posts where category_id=$1")
                .bind(category_id)
                .fetch_one(&self.pool)
                .await?,
        )
    }

    async fn page(&self, page_number: i32, per_page: i64) -> Result<Vec<Post>> {
        Ok(sqlx::query_as::<_, Post>(
            "select * from posts Order By id Asc limit $1 OFFSET ($2-1)*$1 ",
        )
        .bind(per_page)
        .bind(page_number)
        .fetch_all(&self.pool)
        .await?)
    }

    async fn page_for_category(
        &self,
        category_id: i32,
        page_number: i32,
        per_page: i64,
    ) -> Result<Vec<PostWithCategory>> {
        Ok(sqlx::query_as::<_, PostWithCategory>(
            "select posts.title,posts.id,posts.description,categories.name  from posts,categories_posts,categories  where categories_posts.post_id=posts.id and categories.id=categories_posts.category_id and categories_posts.category_id=$1 Order By posts.id Asc  limit $3 offset($2-1)*$3"
        )
        .bind(category_id)
        .bind(page_number)
        .bind(per_page)
        .fetch_all(&self.pool)
        .await?)
    }

    async fn find(&self, id: i32) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_as::<_, Post>("select id, title, description from posts  WHERE id=$1")
                .bind(id)
                .fetch_all(&self.pool)
                .await?,
        )
    }

    async fn category_id_for_post(&self, post_id: i32) -> Result<i32> {
        let category_id: Option<i32> =
            sqlx::query_scalar("select category_id from categories_posts where post_id=$1")
                .bind(post_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(category_id.unwrap_or_default())
    }

    async fn create(&self, new_post: &NewPost) -> Result<()> {
        sqlx::query("insert into posts(title,description) values ($1,$2)")
            .bind(&new_post.title)
            .bind(&new_post.description)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_with_category(&self, new_post: &NewPost, category_id: i32) -> Result<()> {
        let id: i32 =
            sqlx::query_scalar("insert into posts(title,description) values($1,$2) returning id")
                .bind(&new_post.title)
                .bind(&new_post.description)
                .fetch_one(&self.pool)
                .await?;

        sqlx::query("insert into categories_posts values ($1,$2)")
            .bind(id)
            .bind(category_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()> {
        sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
            .bind(&updated.title)
            .bind(&updated.description)
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("update categories_posts set category_id=$2 where post_id=$1")
            .bind(id)
            .bind(category_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_without_category(&self, id: i32, updated: &NewPost) -> Result<()> {
        sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
            .bind(&updated.title)
            .bind(&updated.description)
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("delete from categories_posts where post_id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn attach_category(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()> {
        sqlx::query("update posts set title=$1 ,description=$2 where id=$3")
            .bind(&updated.title)
            .bind(&updated.description)
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("insert into categories_posts values ($1,$2)")
            .bind(id)
            .bind(category_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("delete from categories_posts where post_id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        sqlx::query("delete from posts where id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
