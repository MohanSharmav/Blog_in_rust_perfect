use crate::model::database::{select_posts, Posts};
use actix_web::{web, Error as ActixError};
use anyhow::anyhow;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize, Copy, Clone)]
pub struct PaginationParams {
    pub(crate) page: i32,
    pub per_page: i32,
}

#[derive(Debug)]
pub struct MyError {
    error: ActixError,
}

impl std::convert::From<ActixError> for MyError {
    fn from(error: ActixError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("An error occurred: \"{}\"", self.error))
    }
}

pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let _end_index = start_index + per_page;
    items
        .into_iter()
        .skip(start_index as usize)
        .take(per_page as usize)
        .collect()
}

pub async fn pagination_logic(
    params: web::Query<PaginationParams>,
    db: &Pool<Postgres>,
) -> Result<Vec<Posts>, anyhow::Error> {
    let page = params.page;
    let per_page = params.per_page;
    if page<1 && per_page<1
    {
        Err(anyhow!("Invalid page"))?
    };
    let posts_pagination: Vec<Posts> = select_posts(db).await?;
    let paginated_users = paginate(posts_pagination, page, per_page);
    Ok(paginated_users)
}
