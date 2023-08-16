use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::set_posts_per_page;
use crate::controllers::helpers::pagination_logic::admin_categories;
use crate::controllers::helpers::pagination_logic::general_category;
use crate::model::categories::category_pagination_logic;
use crate::model::categories::{
    all_categories_db, category_db, create_new_category_db, delete_category_db,
    get_all_categories_db, get_specific_category_posts, update_category_db,
};
use crate::model::structs::CreateNewCategory;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::result;

pub async fn get_all_categories(
    config: web::Data<Configuration>,
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
    let total_posts_length = get_pagination_for_all_categories_list(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let posts_per_page_constant = set_posts_per_page().await;
    let param = params.into_inner();
    let count_of_number_of_pages = pages_count.len();
    let current_page: usize = param.clone() as usize;
    if current_page <= 0 || current_page > count_of_number_of_pages {
        let pagination_final_string = admin_categories(current_page, count_of_number_of_pages)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_category = all_categories_db(db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_categories = get_all_categories_db(db, param, posts_per_page_constant)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let _html = handlebars
            .render(
                "admin_category_table",
                &json!({ "pagination":pagination_final_string,"z": &all_categories,"o":all_category,"pages_count":pages_count}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/categories/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string = admin_categories(current_page, count_of_number_of_pages)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_category = all_categories_db(db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_categories = get_all_categories_db(db, param, posts_per_page_constant)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "admin_category_table",
                &json!({ "pagination":pagination_final_string,"z": &all_categories,"o":all_category,"pages_count":pages_count}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    }
}

pub async fn new_category(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_category", &json!({"o":"ax","o":all_category}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn create_category(
    form: web::Form<CreateNewCategory>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = &form.name;
    let db = &config.database_connection;
    create_new_category_db(db, name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn destroy_category(
    id: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<Redirect, actix_web::Error> {
    let to_delete_category = &id.into_inner();
    let db = &config.database_connection;
    delete_category_db(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/categories/page/1"))
}

pub async fn edit_category(
    config: web::Data<Configuration>,
    to_be_updated_category: web::Path<i32>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }

    let db = &config.database_connection;
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let to_be_updated_category = to_be_updated_category.clone();
    let x = get_specific_category_posts(to_be_updated_category, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category ,"o":all_category,"category_old_name":x}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_category(
    id: web::Path<i32>,
    form: web::Form<CreateNewCategory>,
    current_category_name: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let _current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let category_id = id.into_inner();

    update_category_db(name, category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn get_category_posts(
    info: web::Path<(String, u32)>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let path = info.clone().0;
    let par = info.into_inner().1 as i32;
    let category_input: String = path;
    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let count_of_number_of_pages = pages_count.len();
    let current_page: usize = par.clone() as usize;
    let admin = false;

    let pagination_final_string = general_category(
        current_page,
        count_of_number_of_pages,
        &category_input,
        admin,
    )
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

pub async fn get_pagination_for_all_categories_list(
    db: &Pool<Postgres>,
) -> result::Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM categories")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<result::Result<i64, actix_web::Error>> = rows
        .into_iter()
        .map(|row| {
            let final_count: i64 = row
                .try_get("count")
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok::<i64, actix_web::Error>(final_count)
        })
        .collect();

    let before_remove_error = counting_final
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let exact_value = before_remove_error
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*exact_value)
}
