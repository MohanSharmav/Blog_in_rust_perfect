use crate::controller::constants::ConfigurationConstants;
use crate::model::category_database::get_all_categories_database;
use crate::model::database::CreateNewPost;
use crate::model::posts_database::{create_post_database, create_post_without_category_database, delete_post_database, get_category_id_from_post_id, update_post_database, update_post_without_category_database};
use crate::model::single_posts_database::query_single_post_in_struct;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use swagger::BodyExt;

pub async fn get_new_post(
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
    let all_categories = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
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
pub async fn new_posts(
    form: web::Form<CreateNewPost>,
    config: web::Data<ConfigurationConstants>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;
    if category_id.clone() == 0_i32 {
        create_post_without_category_database(title.clone(), description.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Redirect::to("/admin/posts/page/1"))
    } else {
        create_post_database(title.clone(), description.clone(), &category_id.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Redirect::to("/admin/posts/page/1"))
    }
}

pub async fn destroy_post(
    to_delete: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let to_delete = to_delete.into_inner();
    delete_post_database(to_delete, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Redirect::to("/admin/posts/page/1"))
}

pub async fn edit_post(
    id: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    to_be_updated_post: web::Path<String>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let to_be_updated_post = to_be_updated_post.clone();
    update_post_helper(&to_be_updated_post).await;
    let db = &config.database_connection;
    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let post_id = id.into_inner();
    let single_post_struct = query_single_post_in_struct(post_id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_post",
            &json!({ "current_post":single_post_struct,"to_be_updated_post": &to_be_updated_post,"o":all_category }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_post_helper(ids: &String) -> &String {
    ids
}
pub async fn update_post(
    id: web::Path<i32>,
    form: web::Form<CreateNewPost>,
    _current_post_name: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = id.into_inner();
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;

    let get_category_id_of_current_post=get_category_id_from_post_id(id,db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError);

    if category_id.clone() == 0_i32 {
        update_post_without_category_database(title.clone(), description.clone(), id.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
println!("-----------------------------😮");

        Ok(HttpResponse::SeeOther()
            // .insert_header(http::header::LOCATION, "/login")
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        println!("------------------{}{}{}----{}",title,description,id,category_id);
        update_post_database(title, description, id, category_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            // .insert_header(http::header::LOCATION, "/login")
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    }
}
