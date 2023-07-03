use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::http::header::ContentType;
use actix_web::{http, web, HttpResponse, ResponseError};
use handlebars::Handlebars;
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::{Debug, Display, Formatter};
use actix_identity::Identity;
use actix_web::web::Query;
use anyhow::anyhow;
use warp::http::status;
use crate::controller::authentication::login::check_user;
use crate::model::authentication::login_database::LoginCheck;

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
pub async fn pagination_display(
    // params: web::Query<PaginationParams>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    mut params: Option<Query<PaginationParams>>,

) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length: f64 = perfect_pagination_logic(db).await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    println!("a----------------------------");
    check_user(user);
    let pari = params.get_or_insert(Query(PaginationParams::default()));
let current_pag=pari.0;
    let current_page = current_pag.page;
    // if current_page < 1 {
    //     Err(|e|actix_web::error::ErrorInternalServerError(e))?
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
