use crate::controller::common_controller::set_posts_per_page;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::get_pagination_for_all_categories_list;
use crate::model::category_database::{
    category_pagination_controller_database_function, create_new_category_database,
    delete_category_database, get_all_categories_database,
    get_all_categories_database_with_pagination_display, update_category_database,
};
use crate::model::database::{Categories, CreateNewCategory};
use crate::model::pagination_database::{category_pagination_logic, PaginationParams};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::{Query, Redirect};
use actix_web::{http, web, HttpResponse};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_all_categories_controller(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    // mut params: Option<Query<PaginationParams>>,
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
    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_pag = pari.0;
    // let _current_page = current_pag.page;

    // let par=params.unwrap_or_else(1);
    // let parii=par.page;
    let posts_per_page_constant = set_posts_per_page().await;
    let param = params.into_inner();
    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // get_all_categories_database
    // let all_categories = get_all_categories_database(db,)
    //     .await
    //     .map_err(actix_web::error::ErrorInternalServerError)?;
    let all_categories =
        get_all_categories_database_with_pagination_display(db, param, posts_per_page_constant)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "all_categories",
            &json!({ "z": &all_categories,"o":all_category,"pages_count":pages_count}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_new_category(
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

pub async fn receive_new_category(
    form: web::Form<CreateNewCategory>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<Redirect, actix_web::Error> {
    // if user.is_none() {
    //     return Ok(HttpResponse::SeeOther()
    //         .insert_header((http::header::LOCATION, "/"))
    //         .body(""));
    // }
    let name = &form.name;
    let db = &config.database_connection;
    create_new_category_database(db, name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/posts/page/1"))
}

pub async fn delete_category(
    id: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<Redirect, actix_web::Error> {
    // if user.is_none() {
    //     return Ok(HttpResponse::SeeOther()
    //         .insert_header((http::header::LOCATION, "/"))
    //         .body(""));
    // }
    let to_delete_category = &id.into_inner();
    let db = &config.database_connection;
    delete_category_database(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/posts/page/1"))

    //
    // let success_message = "the category deleted successfully";
    // let html = handlebars
    //     .render("message_display", &json!({ "message": success_message }))
    //     .map_err(actix_web::error::ErrorInternalServerError)?;
    //
    // Ok(HttpResponse::Ok()
    //     .content_type(ContentType::html())
    //     .body(html))
}

pub async fn page_to_update_category(
    config: web::Data<ConfigurationConstants>,
    to_be_updated_category: web::Path<String>,
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
    let html = handlebars
        .render(
            "update_category",
            &json!({ "to_be_updated_post": &to_be_updated_category ,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn receive_updated_category(
    id: web::Path<i32>,
    form: web::Form<CreateNewCategory>,
    current_category_name: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let _current_post_name = &current_category_name.into_inner();
    let name = &form.name;
    let category_id = id.into_inner();
    update_category_database(name, category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Redirect::to("/admin/posts/page/1"))
    // let success_message = "the post created successfully";
    // let html = handlebars
    //     .render("message_display", &json!({ "message": success_message }))
    //     .map_err(actix_web::error::ErrorInternalServerError)?;
    //
    // Ok(HttpResponse::Ok()
    //     .content_type(ContentType::html())
    //     .body(html))
}

pub async fn get_category_with_pagination(
    // path: web::Path<String>,
    // params: web::Query<PaginationParams>,
    info: web::Path<(String, u32)>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let path = info.clone().0;
    let mut par = info.into_inner().1 as i32;
    // let category_input: String = path.();
    let category_input: String = path;

    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // let par=params.page;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;

    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
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
            &json!({"tiger":&category_postinng,"pages_count":&pages_count,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
