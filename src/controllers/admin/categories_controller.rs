use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::controllers::helpers::pagination_logic::admin_categories;
use crate::model::categories;
use crate::model::categories::{
    create_new_category_db, delete_category_db, get_all_categories_db, get_specific_category_posts,
    update_category_db,
};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use anyhow::Result;
use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;
use std::fmt::Write;
use validator::Validate;

pub async fn get_all_categories(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    param: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }

    let db = &config.database_connection;
    let total_posts = categories::categories_count(db).await? + 2;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    // calculate the count of pages  ex:- total categories are 15 /3 =5
    // here 5 is total_pages_count
    let total_pages_count = (total_posts / posts_per_page_constant) as usize;
    let current_page = param.into_inner();
    let mut error_html = String::new();
    // receive error messages from post method (check using loops)-> send to html pages
    for message in flash_message.iter() {
        writeln!(error_html, "{}", message.content())
            .map_err(actix_web::error::ErrorInternalServerError)?;
    }
    return if current_page == 0 || current_page > total_pages_count as i32 {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/categories/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string = admin_categories(current_page as usize, total_pages_count)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let all_categories = get_all_categories_db(db, current_page, posts_per_page_constant)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "admin_category_table",
                &json!({"message": error_html,"pagination":pagination_final_string,"categories": &all_categories}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    };
}

pub async fn new_category(
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }

    let html = handlebars
        .render("new_category", &json!({}))
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
    // this function will check the user input with the struct
    // and will get the error message from the struct
    let form_result = form.validate();
    let mut validation_errors = Vec::new();
    let mut flash_error_string = String::new();
    // check if form_result return error
    // use this loop and get all the errors from the struct
    if let Err(errors) = form_result {
        for error in errors.field_errors() {
            validation_errors.push(format!("{}: {:?}", error.0, error.1));
            let error_string = errors.to_string();
            flash_error_string = error_string;
        }
    }
    // if validation_errors is not empty it will be filled with error message from struct
    if !validation_errors.is_empty() {
        FlashMessage::error(flash_error_string).send();

        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/categories/page/1"))
            .finish());
    }

    create_new_category_db(db, name)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    return Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish());
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
    category_to_update: web::Path<i32>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }

    let db = &config.database_connection;
    let category_to_update = *category_to_update;

    let category_old_name = get_specific_category_posts(category_to_update, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_category",
            &json!({ "category_old_name": category_old_name }),
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
    let form_result = form.validate();
    let mut validation_errors = Vec::new();
    let mut flash_error_string = String::new();

    if let Err(errors) = form_result {
        for error in errors.field_errors() {
            validation_errors.push(format!("{}: {:?}", error.0, error.1));
            let error_string = errors.to_string();
            flash_error_string = error_string;
        }
    }

    if !validation_errors.is_empty() {
        FlashMessage::error(flash_error_string).send();
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/categories/page/1"))
            .finish());
    }

    update_category_db(name, category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow, Validate)]
pub struct CreateNewCategory {
    // #[validate --> will check the user input with this condition
    // message the error message is to be prompted when the validation fail
    #[validate(length(
        min = 2,
        message = "category name cannot be empty and minimum should have 2 characters"
    ))]
    pub(crate) name: String,
}
