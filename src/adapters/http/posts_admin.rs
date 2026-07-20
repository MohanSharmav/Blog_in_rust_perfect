use crate::adapters::http::auth_guard::require_login;
use crate::adapters::http::flash::render_flash_messages;
use crate::adapters::http::pagination_view::{self, admin_category_posts, admin_main_page};
use crate::adapters::http::state::AppState;
use crate::adapters::http::validated_form::validate_or_redirect;
use crate::application::ports::CategoryRepository;
use crate::application::posts_service;
use crate::domain::post::NewPost;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{http, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_new_post(
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let all_categories = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "new_post",
            &json!({ "all_categories": &all_categories,"o":all_categories }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn new_post(
    form: web::Form<NewPost>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = validate_or_redirect(&*form, "/admin/posts/page/1") {
        return Ok(redirect);
    }

    posts_service::create_post(&state.posts, &form)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/admin/posts/page/1"))
        .finish())
}

pub async fn destroy_post(
    to_delete: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<Redirect, actix_web::Error> {
    let id: i32 = to_delete
        .parse()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    posts_service::delete_post(&state.posts, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Redirect::to("/admin/posts/page/1"))
}

pub async fn edit_post(
    id: web::Path<i32>,
    state: web::Data<AppState>,
    to_be_updated_post: web::Path<String>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let to_be_updated_post = to_be_updated_post.into_inner();
    let post_id = id.into_inner();

    let edit_view = posts_service::post_for_edit(&state.posts, &state.categories, post_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "update_post",
            &json!({"category_info": edit_view.category_info,"current_post":edit_view.post,"to_be_updated_post": &to_be_updated_post,"o":edit_view.other_categories }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn update_post(
    id: web::Path<i32>,
    form: web::Form<NewPost>,
    _current_post_name: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = id.into_inner();

    if let Some(redirect) = validate_or_redirect(&*form, "/admin/posts/page/1") {
        return Ok(redirect);
    }

    posts_service::update_post(&state.posts, id, &form)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/admin/posts/page/1"))
        .content_type(ContentType::html())
        .finish())
}

pub async fn get_categories_posts(
    info: web::Path<(String, i32)>,
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let (category_input, page_number) = info.into_inner();
    let category_id: i32 = category_input
        .parse()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let Some(listing) =
        posts_service::list_posts_for_category(&state.posts, category_id, page_number as i64)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        let redirect_url = format!("/admin/categories/{category_input}/page/1");
        return Ok(pagination_view::redirect(&redirect_url));
    };

    let pagination_final_string =
        admin_category_posts(listing.page.current, listing.page.total, &category_input);
    let pages_count: Vec<_> = (1..=listing.page.total).collect();

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_separate_categories",
            &json!({"pagination":pagination_final_string,"tiger":&listing.items,"pages_count":&pages_count,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn show_post(
    path: web::Path<String>,
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.parse::<i32>().unwrap_or_default();

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = posts_service::get_post(&state.posts, post_id)
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
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
    params: web::Path<i32>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(redirect) = require_login(&user) {
        return Ok(redirect);
    }
    let page_number = params.into_inner() as i64;

    let Some(listing) = posts_service::list_posts(&state.posts, page_number)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        return Ok(pagination_view::redirect("/admin/posts/page/1"));
    };

    let pages_count: Vec<_> = (1..=listing.page.total).collect();
    let error_html = render_flash_messages(&flash_message)?;

    let pagination_final_string = admin_main_page(listing.page.current, listing.page.total);

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("admin_post_table", &json!({"message": error_html,"tt":&listing.total_items,"pages_count":pages_count,"tiger":listing.items,"o":all_category,"pagination":pagination_final_string}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
