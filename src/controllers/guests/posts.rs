use crate::controllers::admin::posts_controller::number_posts_count;
use crate::controllers::constants::Configuration;
use crate::controllers::helpers::pagination_logic::{general_category, index_pagination};
use crate::model::categories::{all_categories_db, category_db, category_pagination_logic};
use crate::model::posts::{query_single_post, single_post_db, specific_page_posts};
use actix_http::header::LOCATION;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

// @desc    Redirect user to index
// @route   GET /
// // @access  Public
// #[post("/my-api")]
// #[doc("My API description")]
// #[openapi(description = "My API description for Swagger")]
pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub const SET_POSTS_PER_PAGE: i64 = 3;

pub async fn index(
    params: web::Path<i32>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = number_posts_count(db).await?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let param = params.into_inner();
    let current_page = param as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let sample: Vec<_> = (1..=posts_per_page).collect();
    let count_of_number_of_pages = pages_count.len();
    let current_page: usize = current_page;

    if current_page > count_of_number_of_pages || current_page == 0 {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }

    let pagination_final_string = index_pagination(current_page, count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = specific_page_posts(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"pagination":pagination_final_string,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"new_pagination":sample}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn index_redirect(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = number_posts_count(db).await?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let param = 1;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();

    let exact_posts_only = specific_page_posts(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"current_page":param}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn show_posts(
    path: web::Path<String>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = single_post_db(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "single",
            &json!({"o":&single_post,"single_post":single_post_struct,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_category_posts(
    info: web::Path<(String, u32)>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> anyhow::Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let path = info.clone().0;
    let par = info.into_inner().1 as i32;
    let category_input: String = path;
    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let mut count_of_number_of_pages = pages_count.len();
    let current_page: usize = par as usize;
    let admin = false;
    if count_of_number_of_pages == 0 {
        count_of_number_of_pages = 1;
    }
    if current_page == 0 || current_page > count_of_number_of_pages {
        let redirect_url =
            "/posts/category/".to_string() + &*category_input.clone() + &*"/page/1".to_string();

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url))
            .content_type(ContentType::html())
            .finish());
    } else {
        let pagination_final_string =
            general_category(current_page, count_of_number_of_pages, &category_input)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        let category_postinng =
            category_db(category_input.to_string(), db, par, posts_per_page_constant)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_category = all_categories_db(db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "category",
                &json!({"pagination":pagination_final_string,"tiger":&category_postinng,"pages_count":&pages_count,"o":all_category}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    }
}
