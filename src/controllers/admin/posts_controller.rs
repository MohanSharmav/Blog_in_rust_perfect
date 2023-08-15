use crate::controllers::constants::Configuration;
use crate::controllers::guests::posts::set_posts_per_page;
use crate::controllers::helpers::pagination_logic::{
    admin_main_page, admin_category_posts,
};
use crate::model::categories::{
    category_db, category_pagination_logic,
    all_categories_db,
};
use crate::model::posts::single_post_db;
use crate::model::posts::{
    create_post, create_post_without_category, delete_post_db,
    category_id_from_post_id, query_single_post, specific_page_posts,
    update_post_db, update_post_without_category,
};
use crate::model::structs::CreateNewPost;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};

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
            &json!({ "all_categories": all_categories,"o":all_category }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
pub async fn new_posts(
    form: web::Form<CreateNewPost>,
    config: web::Data<Configuration>,
) -> Result<Redirect, actix_web::Error> {
    let db = &config.database_connection;
    let title = &form.title;
    let description = &form.description;
    let category_id = &form.category_id;
    if category_id.clone() == 0_i32 {
        create_post_without_category(title.clone(), description.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Redirect::to("/admin/posts/page/1"))
    } else {
        create_post(title.clone(), description.clone(), &category_id.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Redirect::to("/admin/posts/page/1"))
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
    let to_be_updated_post = to_be_updated_post.clone();
    let db = &config.database_connection;
    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let post_id = id.into_inner();
    let single_post_struct = single_post_db(post_id, db)
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

    let _get_category_id_of_current_post = category_id_from_post_id(id, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError);

    if category_id.clone() == 0_i32 {
        update_post_without_category(title.clone(), description.clone(), id.clone(), db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .content_type(ContentType::html())
            .finish())
    } else {
        println!(
            "------------------{}{}{}----{}",
            title, description, id, category_id
        );
        update_post_db(title, description, id, category_id, db)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::SeeOther()
            // .insert_header(http::header::LOCATION, "/login")
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

    let all_category = all_categories_db(db)
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
    let cp: usize = params.clone() as usize;

    let pagination_final_string =
        admin_category_posts(cp, count_of_number_of_pages, category_input.clone())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_postinng = category_db(
        category_input,
        db,
        params,
        posts_per_page_constant,
    )
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
    let titles = path.parse::<i32>().unwrap_or_default();
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = single_post_db(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_single_post",
            &json!({"o":&single_post,"single_post":single_post_struct,"o":all_category}),
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
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let total_posts_length = number_posts_count(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;

    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let current_page = params.clone();
    let par = params.into_inner();
    let count_of_number_of_pages = pages_count.len();
    let cp: usize = par.clone() as usize;

    let pagination_final_string = admin_main_page(cp, count_of_number_of_pages)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = specific_page_posts(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = all_categories_db(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("admin_post_table", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"pagination":pagination_final_string}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
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

    let a = counting_final
        .get(0)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("error-1"))?;

    let b = a
        .as_ref()
        .map_err(|_er| actix_web::error::ErrorInternalServerError("error-2"))?;

    Ok(*b)
}