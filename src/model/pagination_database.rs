use crate::model::database::{select_posts, Posts};
use actix_web::Error as ActixError;
use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub struct PaginationParams {
    pub(crate) page: Option<i32>,
    per_page: Option<i32>,
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

pub async fn pagination_logic(params: web::Query<PaginationParams>) -> Result<Vec<Posts>, anyhow::Error> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);
    let posts_pagination: Vec<Posts> = select_posts().await?;
    let paginated_users = paginate(posts_pagination.clone(), page, per_page);
    let _posts_per_page_length = posts_pagination.len();
    Ok(paginated_users)
}
