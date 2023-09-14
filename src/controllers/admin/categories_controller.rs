use crate::controllers::constants::Configuration;
use crate::controllers::helpers::pagination_logic::admin_categories;
use crate::model::categories;
use crate::model::categories::{
    create_new_category_db, delete_category_db, get_all_categories_db, get_specific_category_posts,
    update_category_db,
};
use crate::SET_POSTS_PER_PAGE;
use actix_identity::Identity;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use anyhow::Result;
use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

pub async fn get_all_categories(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    current_page: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let total_posts = categories::categories_count(db).await?;
    // set_posts_per_page -1 because
    // if number of posts is 13 then
    // 13/3= 4 pages but it should be 5
    // so 13+ "2" = 15 /3 is which makes 5 pages so constant-1 is perfect logic
    // calculate the count of pages  ex:- total categories are 15 /3 =5
    // here 5 is total_pages_count
    let total_pages_count = (total_posts + *SET_POSTS_PER_PAGE - 1) / *SET_POSTS_PER_PAGE;
    let current_page = current_page.into_inner();
    let mut error_html = String::new();
    // receive error messages from post method (check using loops)-> send to html pages
    flash_message
        .iter()
        .for_each(|message| error_html.push_str(message.content()));

    if current_page == 0 || current_page > total_pages_count as i32 {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/categories/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string =
            admin_categories(current_page as usize, total_pages_count as usize)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        let all_categories = get_all_categories_db(db, current_page, *SET_POSTS_PER_PAGE)
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
    }
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
    let category_name = &form.name;
    let db = &config.database_connection;
    // this function will check the user input with the struct
    // and validate the from
    let form_result = form.validate();
    // if form_result is result type --> if it returns ValidationError then
    // this error shall be passed to the front end using actix message
    // error shall be converted to string and passed
    if let Err(errors) = form_result {
        FlashMessage::error(errors.to_string()).send();

        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/categories/page/1"))
            .finish());
    }
    create_new_category_db(db, category_name)
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
    let to_delete_category = id.into_inner();
    let db = &config.database_connection;
    delete_category_db(db, to_delete_category)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Redirect::to("/admin/categories/page/1"))
}

pub async fn edit_category(
    config: web::Data<Configuration>,
    category_id: web::Path<i32>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let category_id = *category_id;
    let category_old_name = get_specific_category_posts(category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let html = handlebars
        .render(
            "update_category",
            &json!({ "category_old_name": category_old_name,"category_to_update":category_id }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_category(
    id: web::Path<i32>,
    form: web::Form<CreateNewCategory>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let new_category_name = &form.name;
    let category_id = id.into_inner();
    let form_result = form.validate();
    if let Err(errors) = form_result {
        FlashMessage::error(errors.to_string()).send();

        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
            .finish());
    }

    update_category_db(new_category_name, category_id, db)
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
    // .validate() will check the message
    // this same message is sent front end with flash-message
    #[validate(length(
        min = 2,
        message = "category name cannot be empty and minimum should have 2 characters"
    ))]
    pub(crate) name: String,
}
