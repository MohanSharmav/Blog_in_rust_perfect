use crate::controller::authentication::login::check_user;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::authentication::login_database::LoginCheck;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
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
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    mut params: Option<Query<PaginationParams>>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let total_posts_length: f64 = perfect_pagination_logic(db).await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let pari = params.get_or_insert(Query(PaginationParams::default()));
    let current_pag = pari.0;
    let current_page = current_pag.page;
    // if current_page < 1 {
    //     Err(|e|actix_web::error::ErrorInternalServerError)?
    // };
    let paginators = pagination_logic(params, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = select_specific_pages_post(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("admin_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
