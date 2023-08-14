use crate::controller::admin::admin_pagination::admin_pagination_with_category;
use crate::controller::admin::pagination_controller::get_pagination_for_all_categories_list;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::guests::common_controller::set_posts_per_page;
use crate::controller::guests::General_pagination::general_pagination_with_category;
use crate::model::category::{
    category_pagination_controller_database_function, create_new_category_database,
    delete_category_database, get_all_categories_database,
    get_all_categories_database_with_pagination_display, get_all_specific_category_database,
    update_category_database,
};
use crate::model::category::category_pagination_logic;
use crate::model::structs::CreateNewCategory;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse, HttpResponseBuilder};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_all_categories(
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
    let cp: usize = param.clone() as usize;

    let pagination_final_string = admin_pagination_with_category(cp, count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories =
        get_all_categories_database_with_pagination_display(db, param, posts_per_page_constant)
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

pub async fn new_category(
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
    let all_category = get_all_categories_database(db)
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
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = &form.name;
    let db = &config.database_connection;
    create_new_category_database(db, name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn destroy_category(
    id: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
) -> Result<Redirect, actix_web::Error> {
    let to_delete_category = &id.into_inner();
    let db = &config.database_connection;
    delete_category_database(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/categories/page/1"))
}

pub async fn edit_category(
    config: web::Data<ConfigurationConstants>,
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
    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let to_be_updated_category = to_be_updated_category.clone();
    let x = get_all_specific_category_database(to_be_updated_category, db)
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
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let _current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let category_id = id.into_inner();
    update_category_database(name, category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn get_category_posts(
    info: web::Path<(String, u32)>,
    config: web::Data<ConfigurationConstants>,
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
    let cp: usize = par.clone() as usize;
    let admin = false;

    let pagination_final_string =
        general_pagination_with_category(cp, count_of_number_of_pages, &category_input, admin)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_postinng = category_pagination_controller_database_function(
        category_input.to_string(),
        db,
        par,
        posts_per_page_constant,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
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
