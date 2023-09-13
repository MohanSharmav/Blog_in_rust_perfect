use crate::controllers::constants::Configuration;
use crate::controllers::helpers::pagination_logic::{admin_category_posts, admin_main_page};
use crate::model::categories::{
    all_categories_db, all_categories_exclusive, category_based_posts_db,
    get_specific_category_posts, individual_category_posts_count,
};
use crate::model::posts;
use crate::model::posts::{
    category_id_from_post_id, create_post, create_post_without_category, delete_post_db,
    specific_page_posts, update_post_db, update_post_without_category,
};
use crate::model::posts::{single_post_db, update_post_from_no_category};
use crate::SET_POSTS_PER_PAGE;
use actix_identity::Identity;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate};

pub async fn get_new_post(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let all_categories = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("new_post", &json!({ "categories": all_categories }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn new_post(
    form: web::Form<CreateNewPost>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;
    let form_result = form.validate();

    match form_result {
        Ok(_) => {
            if *category_id == 0_i32 {
                create_post_without_category(title.clone(), description.clone(), db)
                    .await
                    .map_err(actix_web::error::ErrorInternalServerError)?;

                Ok(HttpResponse::SeeOther()
                    .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
                    .finish())
            } else {
                create_post(title.clone(), description.clone(), &category_id.clone(), db)
                    .await
                    .map_err(actix_web::error::ErrorInternalServerError)?;

                Ok(HttpResponse::SeeOther()
                    .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
                    .finish())
            }
        }
        Err(errors) => {

            FlashMessage::error(errors.to_string()).send();
            return Ok(HttpResponse::SeeOther()
                .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
                .finish());
        }
    }
}

pub async fn destroy_post(
    post_id: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let post_id = post_id.into_inner();

    delete_post_db(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Redirect::to("/admin/posts/page/1"))
}

pub async fn edit_post(
    post_id: web::Path<i32>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let post_id = post_id.into_inner();

    let single_post_struct = single_post_db(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_id = category_id_from_post_id(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_info = get_specific_category_posts(category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_categories = all_categories_exclusive(db, category_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_post",
            &json!({"category_info": category_info,"current_post":single_post_struct,"categories":all_categories}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_post(
    id: web::Path<i32>,
    form: web::Form<CreateNewPost>,
    _current_post_name: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = id.into_inner();
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;
    // let mut validation_errors = Vec::new();
    let form_result = form.validate();
    // let mut flash_errors_string = String::new();

    if let Err(errors) = form_result {
        FlashMessage::error(errors.to_string()).send();

        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
            .finish());
    }

    let category_id_of_current_post = category_id_from_post_id(post_id, db)
        .await
        .unwrap_or_default();
    // update post from no category to no category
    // no category id = 0
    if category_id_of_current_post == 0 && *category_id == 0_i32 {
        update_post_without_category(title.clone(), description.clone(), post_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }
    // update post from no category to category
    if category_id_of_current_post == 0 {
        update_post_from_no_category(title, description, category_id, post_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }
    // update post from category to no category
    if *category_id == 0_i32 {
        update_post_without_category(title.clone(), description.clone(), post_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        // update post from one category to another category
        update_post_db(title, description, post_id, category_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    }
}

pub async fn categories_based_posts(
    params: web::Path<(String, i32)>,
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
    let category_id: String = params.clone().0;
    let current_page: usize = params.into_inner().1 as usize;

    let total_posts = individual_category_posts_count(&category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let total_pages_count = (total_posts + *SET_POSTS_PER_PAGE - 1) / *SET_POSTS_PER_PAGE;

    if current_page == 0 || current_page > total_pages_count as usize {
        let redirect_url = "/admin/categories/".to_string() + &category_id + "/page/1";

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string = admin_category_posts(
            current_page,
            total_pages_count as usize,
            category_id.clone(),
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let category_posts =
            category_based_posts_db(category_id, db, current_page as i32, *SET_POSTS_PER_PAGE)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "admin_separate_categories",
                &json!({"pagination":pagination_final_string,"posts":&category_posts}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    }
}

pub async fn admin_index(
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
    let total_posts = posts::number_posts_count(db).await?;
    let total_pages_count = (total_posts + *SET_POSTS_PER_PAGE - 1) / *SET_POSTS_PER_PAGE;
    let current_page = current_page.into_inner();
    let mut error_html = String::new();

    flash_message
        .iter()
        .for_each(|message| error_html.push_str(message.content()));
    // if the user enters the wrong page number which is less than the 1 or greater total_pages_count
    // then he will be redirected to the the page 1
    if current_page == 0 || current_page > total_pages_count as i32 {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string =
            admin_main_page(current_page as usize, total_pages_count as usize)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        let exact_posts_only = specific_page_posts(current_page, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars.render("admin_post_table", &json!({"message": error_html,"posts":exact_posts_only,"pagination":pagination_final_string}))
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html))
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Post {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct PostsCategory {
    pub title: String,
    pub id: i32,
    pub description: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow, Validate)]
pub struct CreateNewPost {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "description cannot be empty"))]
    pub description: String,
    pub category_id: i32,
}
