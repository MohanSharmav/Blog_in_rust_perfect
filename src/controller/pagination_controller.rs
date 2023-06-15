use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::{pagination_logic, PaginationParams};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::{PgPool, Row};

pub async fn pagination_display(
    params: web::Query<PaginationParams>,
    db: web::Data<PgPool>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let total_posts_length: f64 = perfect_pagination_logic(&db).await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as usize;
    let pages_count :Vec<_>= (1..=posts_per_page).into_iter().collect();

    let paginators = pagination_logic(params.clone(), &db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let _current_page = params.page;
    let exact_posts_only = select_specific_pages_post(_current_page, &db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(&db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("admin_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn perfect_pagination_logic(
    db: &web::Data<PgPool>,
) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(&***db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

     let mut counting_final = 0;

    let _ = rows.iter().map(|row| {
        let title:i64 = row
            .try_get("count")
            .map_err(actix_web::error::ErrorInternalServerError)? ;
        counting_final += title;
        Ok::<i64,actix_web::Error>(counting_final)
    });



    Ok(counting_final)
}

pub async fn category_pagination_logic(
    category_input: &String,
    db: &web::Data<PgPool>,
) -> Result<i64, anyhow::Error> {
    let category_input = category_input.to_string();
    let category_id = category_input.parse::<i32>()?;

    let rows = sqlx::query("SELECT COUNT(*) FROM posts where category_id=$1")
        .bind(category_id)
        .fetch_all(&***db)
        .await?;

    let mut counting_final = 0;
    for row in rows {
        let title  = row.try_get("count")?;
        //Todo
        counting_final = title;
    }
    Ok(counting_final )
}
