use crate::controller::common_controller::set_posts_per_page;
use crate::controller::constants::ConfigurationConstants;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::pagination_logic;
use crate::model::pagination_logic::select_specific_pages_post;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{http, web, HttpResponse, ResponseError};
use handlebars::Handlebars;
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::{Debug, Display, Formatter};
use warp::http::status;
use crate::controller::admin_pagination::admin_pagination_main_page;

#[derive(Debug)]
struct MyOwnErrors {
    status_codes: i32,
}

impl Display for MyOwnErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyOwnErrors: StatusCode {}", self.status_codes)
    }
}

impl ResponseError for MyOwnErrors {
    fn status_code(&self) -> StatusCode {
        status::StatusCode::BAD_GATEWAY
    }
}
pub async fn admin_index(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
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
    let current_page = params.clone();
    let par = params.into_inner();
    let count_of_number_of_pages = pages_count.len();
    let cp: usize = par.clone() as usize;



    let pagination_final_string=admin_pagination_main_page(cp,count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let paginators = pagination_logic(&par, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = select_specific_pages_post(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;


    let htmls = handlebars.render("admin_post_table", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"pagination":pagination_final_string}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn perfect_pagination_logic(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let a = counting_final
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let b = a
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*b)
}

pub async fn get_pagination_for_all_categories_list(
    db: &Pool<Postgres>,
) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM categories")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let a = counting_final
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let b = a
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*b)
}

pub async fn england_admin_pagination_display(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let htmls = handlebars
        .render("admin_category_table", &json!({"SASa":"ASSA"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
