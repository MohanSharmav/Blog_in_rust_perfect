use std::fmt::Error;

use actix_web::{web, HttpResponse, Result, ResponseError};
use serde::Deserialize;
use crate::model::database::{posts, select_posts};

#[derive(Deserialize)]
#[derive(Copy, Clone)]
pub struct PaginationParams {
    pub(crate) page: Option<i32>,
    per_page: Option<i32>,
}

#[derive(Deserialize)]
pub struct Total_pages{
    total_pages_count: Option<i32>,
}


use actix_web::{ App, Error as ActixError, HttpServer};
use futures::TryFutureExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

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
        f.write_fmt(format_args!(
            "An error occurred: \"{}\"",
            self.error.to_string()
        ))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }



}
pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let end_index = start_index + per_page;
    items.into_iter().skip(start_index as usize).take(per_page as usize).collect()
}

pub async fn pagination_logic(params: web::Query<PaginationParams>  ) -> Result<Vec<posts>,MyError>
{

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);
  //  println!("---------------------->{:?}", page);


        // let current_page = page + 2;

  //  println!("---------------------->{}", current_page);
    // select_specific_pages_post(current_page).await.expect("TODO: panic message");


    let mut posts_pagination:Vec<posts>= select_posts().await.expect("maosdso");
    let paginated_users = paginate(posts_pagination.clone(), page, per_page);


    let posts_per_page_length = posts_pagination.len();
    Ok(paginated_users)
}

// pub async fn current_page(params: web::Query<PaginationParams>)->page
// {
//     let page = params.page.unwrap_or(1);
//     page
// }



