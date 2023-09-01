use crate::controllers::constants::Configuration;
use crate::controllers::helpers::pagination_logic::{general_category, index_pagination};
use crate::model::categories::{
    all_categories_db, category_based_posts_db, individual_category_posts_count,
};
use crate::model::posts::number_posts_count;
use crate::model::posts::{single_post_db, specific_page_posts};
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

// @desc    Redirect user to index
// @route   GET /
// @access  Public

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub const SET_POSTS_PER_PAGE: i64 = 3;

pub async fn index(
    param: web::Path<i32>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts = number_posts_count(db).await? + 2;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let total_pages_count = (total_posts / posts_per_page_constant) as usize;
    let current_page = param.into_inner();

    if current_page > total_pages_count as i32 || current_page == 0 {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }

    let pagination_final_string = index_pagination(current_page as usize, total_pages_count)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = specific_page_posts(current_page, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars.render("common", &json!({"pagination":pagination_final_string,"posts":exact_posts_only,"categories":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn show_post(
    path: web::Path<String>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let post_id = path.parse::<i32>().unwrap_or_default();

    let post = single_post_db(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "single_post",
            &json!({"post":post,"categories":all_categories}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_category_based_posts(
    params: web::Path<(String, u32)>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> anyhow::Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let category_id = params.clone().0;
    let current_page = params.into_inner().1 as i32;
    let total_posts_length = individual_category_posts_count(&category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        + 2;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let total_pages_count = total_posts_length / posts_per_page_constant;

    return if current_page == 0 || current_page > total_pages_count as i32 {
        let redirect_url =
            "/posts/category/".to_string() + &*category_id.clone() + &*"/page/1".to_string();

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string = general_category(
            current_page as usize,
            total_pages_count as usize,
            &category_id,
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let category_based_posts = category_based_posts_db(
            category_id.to_string(),
            db,
            current_page,
            posts_per_page_constant,
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_category = all_categories_db(db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "category",
                &json!({"pagination":pagination_final_string,"posts":&category_based_posts,"categories":all_category}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    };
}
