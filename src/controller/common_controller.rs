use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_logic::select_specific_pages_post;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;
//
// pub async fn common_page_controller(
//     mut params: Option<Query<PaginationParams>>,
//     config: web::Data<ConfigurationConstants>,
//     handlebars: web::Data<Handlebars<'_>>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let db = &config.database_connection;
//     let total_posts_length = perfect_pagination_logic(db).await?;
//     let posts_per_page_constant = set_posts_per_page().await as i64;
//     let mut posts_per_page = total_posts_length / posts_per_page_constant;
//     let check_remainder = total_posts_length % posts_per_page_constant;
//     if check_remainder != 0 {
//         posts_per_page += 1;
//     }
//     let posts_per_page = posts_per_page as usize;
//     let pages_count: Vec<_> = (1..=posts_per_page).collect();
//     let pari = params.get_or_insert(Query(PaginationParams::default()));
//     let current_page = pari.clone().page;
//     let exact_posts_only = select_specific_pages_post(current_page, &db.clone())
//         .await
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     let all_category = get_all_categories_database(db)
//         .await
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
//         .map_err( actix_web::error::ErrorInternalServerError)?;
//
//     Ok(HttpResponse::Ok()
//         .content_type(ContentType::html())
//         .body(htmls))
// }

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub async fn set_posts_per_page() -> i32 {
    3
}

pub async fn new_common_page_controller(
    // mut params: Option<Query<PaginationParams>>,
    params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = perfect_pagination_logic(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_page = pari.clone().page;
    let param = params.into_inner();
    // let par=*param as i32;
    let exact_posts_only = select_specific_pages_post(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn new_common_page_controller_test(
    // mut params: Option<Query<PaginationParams>>,
    params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = perfect_pagination_logic(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_page = pari.clone().page;
    let param = params.into_inner();
    // let par=*param as i32;
    let exact_posts_only = select_specific_pages_post(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common_two", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
