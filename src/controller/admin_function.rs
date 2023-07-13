use crate::controller::authentication::login::check_user;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::perfect_pagination_logic;
// use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::authentication::login_database::LoginCheck;
use crate::model::category_database::{
    category_pagination_controller_database_function, get_all_categories_database,
};
use crate::model::pagination_database::{
    category_pagination_logic, pagination_logic, PaginationParams,
};
use crate::model::pagination_logic::select_specific_pages_post;
use crate::model::single_posts_database::{query_single_post, query_single_post_in_struct};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Query;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use anyhow::anyhow;
use handlebars::Handlebars;
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::{Debug, Display, Formatter};
use warp::http::status;

pub async fn admin_category_display(
    path: web::Path<String>,
    _params: web::Query<PaginationParams>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let category_input: String = path.into_inner();
    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let total_posts_length = total_posts_length as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as i32;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    // let category_postinng = category_pagination_controller_database_function(&category_input, db)
    let category_postinng = category_pagination_controller_database_function(category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_categories_page",
            &json!({"tiger":&category_postinng,"pages_count":&pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn admin_unique_posts_display(
    path: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    //Todo
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = query_single_post_in_struct(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_single_post",
            &json!({"o":&single_post,"single_post":single_post_struct}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
