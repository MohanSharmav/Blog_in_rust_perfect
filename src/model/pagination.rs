// use crate::model::structs::{Posts, select_posts};
// use actix_web::Error as ActixError;
// use anyhow::anyhow;
// use serde::Deserialize;
// use sqlx::{Pool, Postgres, Row};
//
// #[derive(Deserialize, Copy, Clone, PartialEq)]
// pub struct PaginationParams {
//     pub(crate) page: i32,
// }
//
// impl Default for PaginationParams {
//     fn default() -> Self {
//         PaginationParams { page: 1 }
//     }
// }
//
// impl std::fmt::Display for PaginationParams {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "PaginationParams(page1: {})", self.page)
//     }
// }
//
// #[derive(Debug)]
// pub struct MyError {
//     error: ActixError,
// }
//
// impl std::convert::From<ActixError> for MyError {
//     fn from(error: ActixError) -> Self {
//         Self { error }
//     }
// }
//
// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("An error occurred: \"{}\"", self.error))
//     }
// }
//
// pub fn paginate<T>(items: Vec<T>, _page: i32) -> Vec<T> {
//     let start_index = 1;
//     let _end_index = start_index + 3;
//     items.into_iter().skip(start_index as usize).collect()
// }
//
// pub async fn pagination_logic(
//     params: &i32,
//     db: &Pool<Postgres>,
// ) -> Result<Vec<Posts>, anyhow::Error> {
//     let page = params;
//     let posts_pagination: Vec<Posts> = select_posts(db).await?;
//     let paginated_users = paginate(posts_pagination, *page);
//     Ok(paginated_users)
// }
