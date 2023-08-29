use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::controllers::helpers::pagination_logic::{admin_category_posts, admin_main_page};
use crate::model::categories::{
    all_categories_db, all_categories_exception, category_db, category_pagination_logic,
    get_specific_category_posts,
};
use crate::model::posts::{
    category_id_from_post_id, create_post, create_post_without_category, delete_post_db,
    specific_page_posts, update_post_db, update_post_without_category,
};
use crate::model::posts::{single_post_db, update_post_from_no_category};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use std::fmt::Write;
use validator::Validate;

pub async fn get_new_post(
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
    let all_categories = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "new_post",
            &json!({ "all_categories": all_categories,"categories":all_category }),
        )
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
    let mut validation_errors = Vec::new();
    let form_result = form.validate();
    let mut flash_errors_string = String::new();

    if let Err(errors) = form_result {
        for error in errors.field_errors() {
            validation_errors.push(format!("{} : {:?}", error.0, error.1));
            let error_string = errors.to_string();
            flash_errors_string = error_string;
        }
    }
    if !validation_errors.is_empty() {
        FlashMessage::error(flash_errors_string).send();
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
            .finish());
    }

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

pub async fn destroy_post(
    to_delete: web::Path<String>,
    config: web::Data<Configuration>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let to_delete = to_delete.into_inner();
    delete_post_db(to_delete, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Redirect::to("/admin/posts/page/1"))
}

pub async fn edit_post(
    id: web::Path<i32>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let post_id = id.into_inner();
    let single_post_struct = single_post_db(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let category_id = category_id_from_post_id(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let category_info = get_specific_category_posts(category_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let all_category = all_categories_exception(db, category_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let html = handlebars
        .render(
            "update_post",
            &json!({"category_info": category_info,"current_post":single_post_struct,"categories":all_category }),
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
    let id = id.into_inner();
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;
    let mut validation_errors = Vec::new();
    let form_result = form.validate();
    let mut flash_errors_string = String::new();

    if let Err(errors) = form_result {
        for error in errors.field_errors() {
            validation_errors.push(format!("{} : {:?}", error.0, error.1));
            let error_string = errors.to_string();
            flash_errors_string = error_string;
        }
    }
    if !validation_errors.is_empty() {
        FlashMessage::error(flash_errors_string).send();
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
            .finish());
    }

    let category_id_of_current_post = category_id_from_post_id(id, db).await.unwrap_or_default();
    if category_id_of_current_post == 0 && *category_id == 0_i32 {
        update_post_without_category(title.clone(), description.clone(), id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }
    if category_id_of_current_post == 0 {
        update_post_from_no_category(title, description, category_id, id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }
    if *category_id == 0_i32 {
        update_post_without_category(title.clone(), description.clone(), id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    } else {
        update_post_db(title, description, id, category_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish());
    }
}

pub async fn get_categories_posts(
    info: web::Path<(String, i32)>,
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
    let category_input: String = info.clone().0;
    let params = info.into_inner().1;

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
    let current_page: usize = params as usize;

    if count_of_number_of_pages == 0 {
        count_of_number_of_pages = 1;
    }
    if current_page == 0 || current_page > count_of_number_of_pages {
        let redirect_url =
            "/admin/categories/".to_string() + &*category_input.clone() + &*"/page/1".to_string();

        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url))
            .content_type(ContentType::html())
            .finish());
    } else {
        let pagination_final_string = admin_category_posts(
            current_page,
            count_of_number_of_pages,
            category_input.clone(),
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let category_postinng = category_db(category_input, db, params, posts_per_page_constant)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let html = handlebars
            .render(
                "admin_separate_categories",
                &json!({"pagination":pagination_final_string,"posts":&category_postinng}),
            )
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html));
    }
}

pub async fn show_post(
    path: web::Path<String>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = single_post_db(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_single_post",
            &json!({"single_post":single_post_struct,"categories":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn admin_index(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let total_posts_length = number_posts_count(db).await?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;

    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let start_page = *params;
    let par = params.into_inner();
    let count_of_number_of_pages = pages_count.len();
    let current_page: usize = par as usize;
    let mut error_html = String::new();
    for message in flash_message.iter() {
        writeln!(error_html, "{}", message.content())
            .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    if current_page == 0 || current_page > count_of_number_of_pages {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        let pagination_final_string = admin_main_page(current_page, count_of_number_of_pages)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let exact_posts_only = specific_page_posts(start_page, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let htmls = handlebars.render("admin_post_table", &json!({"message": error_html,"posts":exact_posts_only,"pagination":pagination_final_string}))
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(htmls))
    }
}

pub async fn number_posts_count(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let counting_final: Vec<Result<i64, actix_web::Error>> = rows
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

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Posts {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct PostsCategories {
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
