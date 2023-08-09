use crate::controller::common_controller::set_posts_per_page;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::get_pagination_for_all_categories_list;
use crate::model::category_database::{
    category_pagination_controller_database_function, create_new_category_database,
    delete_category_database, get_all_categories_database,
    get_all_categories_database_with_pagination_display, get_all_specific_category_database,
    update_category_database,
};
use crate::model::database::CreateNewCategory;
use crate::model::pagination_database::category_pagination_logic;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse, HttpResponseBuilder};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_all_categories_controller(
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
    let x1 = r#"<div class="card mb-4">
                                <!-- Basic Pagination -->
                                   <!-- Basic Pagination -->
                                                <nav aria-label="Page navigation">
                                                    <ul class="pagination">
                                            "#;
    let y = pages_count.len();
    let cp: usize = param.clone() as usize;
    let mut pagination_final_string = String::new();
    pagination_final_string.push_str(x1);
    for i in 1..y + 1 {
        if i == cp {
            let tag_and_url = r#"

<li class="page-item active">
              <a class="page-link "   href="/admin/categories/page/"#;
            pagination_final_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_final_string.push_str(&*href_link);
            let page_constant = r#"">"#;
            pagination_final_string.push_str(page_constant);
            let text_inside_tag = i.to_string();
            pagination_final_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_final_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"

<li class="page-item">
              <a class="page-link "   href="/admin/categories/page/"#;
            pagination_final_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_final_string.push_str(&*href_link);
            let page_constant = r#"">"#;
            pagination_final_string.push_str(page_constant);
            let text_inside_tag = i.to_string();
            pagination_final_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_final_string.push_str(close_tag);
        }
    }

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

pub async fn delete_category(
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

pub async fn page_to_update_category(
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

pub async fn receive_updated_category(
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
        // .insert_header(http::header::LOCATION, "/login")
        .insert_header((LOCATION, "/admin/categories/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn get_category_with_pagination(
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
    let x1 = r#"
    <br>
<div class="paginations">
 "#;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let y = pages_count.len();
    let cp: usize = par.clone() as usize;
    let mut pagination_final_string = String::new();
    pagination_final_string.push_str(x1);
    for i in 1..y + 1 {
        if i == cp {
            let tag_and_url = r#"<a class="active"  href="/posts/category/"#;
            pagination_final_string.push_str(tag_and_url);
            let category_id = category_input.clone();
            pagination_final_string.push_str(&*category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_final_string.push_str(&*static_keyword_page);
            let href_link = i.to_string();
            pagination_final_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_final_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_final_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_final_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/category/"#;
            pagination_final_string.push_str(tag_and_url);
            let category_id = category_input.clone();
            pagination_final_string.push_str(&*category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_final_string.push_str(&*static_keyword_page);
            let href_link = i.to_string();
            pagination_final_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_final_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_final_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_final_string.push_str(close_tag);
        }
    }

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
