use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::SET_POSTS_PER_PAGE;
use crate::controllers::helpers::auth_guard::require_login;
use crate::controllers::helpers::flash::render_flash_messages;
use crate::controllers::helpers::validated_form::validate_or_redirect;
use crate::controllers::helpers::pagination_logic::{
    admin_category_posts, admin_main_page, resolve_current_page, total_pages,
};
use crate::model::categories::{
    all_categories_db, all_categories_exception, category_db, category_pagination_logic,
    get_specific_category_posts,
};
use crate::model::posts::{
    category_id_from_post_id, create_post, create_post_without_category, delete_post_db,
    specific_page_posts, update_post_db, update_post_without_category,
};
use crate::model::posts::{single_post_db, update_post_from_no_category};
use crate::model::structs::CreateNewPost;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use handlebars::Handlebars;
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn get_new_post(
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
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
            &json!({ "all_categories": all_categories,"o":all_category }),
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

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/posts/page/1") {
        return Ok(redirect);
    }

    if *category_id == 0_i32 {
        create_post_without_category(title, description, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        create_post(title, description, category_id, db)
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
    to_be_updated_post: web::Path<String>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let to_be_updated_post = to_be_updated_post.into_inner();
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
            &json!({"category_info": category_info,"current_post":single_post_struct,"to_be_updated_post": &to_be_updated_post,"o":all_category }),
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

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/posts/page/1") {
        return Ok(redirect);
    }
    let category_id_of_current_post = category_id_from_post_id(id, db).await.unwrap_or_default();

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
        update_post_without_category(title, description, id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        update_post_db(title, description, id, category_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    }
}

pub async fn get_categories_posts(
    info: web::Path<(String, i32)>,
    config: web::Data<Configuration>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let db = &config.database_connection;
    let (category_input, params) = info.into_inner();

    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let pages_count: Vec<_> =
        (1..=total_pages(total_posts_length, posts_per_page_constant)).collect();
    let redirect_url = format!("/admin/categories/{category_input}/page/1");
    let (current_page, count_of_number_of_pages) = match resolve_current_page(
        params as i64,
        total_posts_length,
        posts_per_page_constant,
        true,
        &redirect_url,
    ) {
        Ok(resolved) => resolved,
        Err(redirect) => return Ok(redirect),
    };

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
            &json!({"pagination":pagination_final_string,"tiger":&category_postinng,"pages_count":&pages_count,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

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

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = single_post_db(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "single",
            &json!({"single_post":single_post_struct,"o":all_category}),
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
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let db = &config.database_connection;
    let total_posts_length = number_posts_count(db).await?;
    let posts_per_page_constant = SET_POSTS_PER_PAGE;
    let param = params.into_inner();

    let (current_page, count_of_number_of_pages) = match resolve_current_page(
        param as i64,
        total_posts_length,
        posts_per_page_constant,
        false,
        "/admin/posts/page/1",
    ) {
        Ok(resolved) => resolved,
        Err(redirect) => return Ok(redirect),
    };

    let pages_count: Vec<_> = (1..=count_of_number_of_pages).collect();
    let error_html = render_flash_messages(&flash_message)?;

    let pagination_final_string = admin_main_page(current_page, count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = specific_page_posts(current_page as i32, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("admin_post_table", &json!({"message": error_html,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"pagination":pagination_final_string}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn number_posts_count(db: &Pool<Postgres>) -> Result<i64, actix_web::error::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}
