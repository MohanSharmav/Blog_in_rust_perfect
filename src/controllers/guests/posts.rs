use crate::controllers::constants::Configuration;
use crate::controllers::helpers::pagination_logic::{general_category, index_pagination};
use crate::model::categories::{
    all_categories_db, category_based_posts_db, individual_category_posts_count,
};
use crate::model::posts::number_posts_count;
use crate::model::posts::{single_post_db, specific_page_posts};
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{web, HttpResponse, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use handlebars::Handlebars;
use serde_json::json;
use crate::SET_POSTS_PER_PAGE;

// @desc    Redirect user to index
// @route   GET /
// @access  Public

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub async fn index(
    current_page: web::Path<i32>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts = number_posts_count(db).await?;
    let total_pages_count = (total_posts + *SET_POSTS_PER_PAGE- 1) / *SET_POSTS_PER_PAGE;
    let current_page = current_page.into_inner();
    let mut error_html = String::new();
    //display the flash message

    flash_message
        .iter()
        .for_each(|message| error_html.push_str(&message.content().to_string()));

    if current_page > total_pages_count as i32 || current_page == 0 {
        FlashMessage::error("wrong page number").send();

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/posts/page/1"))
            .finish());
    }

    let pagination_final_string =
        index_pagination(current_page as usize, total_pages_count as usize)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = specific_page_posts(current_page, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars.render("common", &json!({"message":error_html,"pagination":pagination_final_string,"posts":exact_posts_only,"categories":all_categories}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn show_post(
    post_id: web::Path<String>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let post_id = post_id
        .parse::<i32>()
        .map_err(actix_web::error::ErrorInternalServerError)?;

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
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let total_pages_count = (total_posts_length + *SET_POSTS_PER_PAGE - 1) / *SET_POSTS_PER_PAGE;

    if current_page == 0 || current_page > total_pages_count as i32 {
        let redirect_url = "/posts/category/".to_string() + &category_id + "/page/1";

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
            *SET_POSTS_PER_PAGE,
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_categories = all_categories_db(db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "category",
                &json!({"pagination":pagination_final_string,"posts":&category_based_posts,"categories":all_categories}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    }
}
