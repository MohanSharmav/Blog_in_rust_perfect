use crate::adapters::http::pagination_view::{self, general_category, index_pagination};
use crate::adapters::http::state::AppState;
use crate::application::ports::CategoryRepository;
use crate::application::posts_service;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub async fn index(
    params: web::Path<i32>,
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let page_number = params.into_inner() as i64;

    let Some(listing) = posts_service::list_posts(&state.posts, page_number)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        return Ok(pagination_view::redirect("/posts/page/1"));
    };

    let pagination_final_string = index_pagination(listing.page.current, listing.page.total);
    let pages_count: Vec<_> = (1..=listing.page.total).collect();

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"pagination":pagination_final_string,"tt":&listing.total_items,"pages_count":&pages_count,"tiger":listing.items,"o":all_category,"new_pagination":&pages_count}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn index_redirect(
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let first_page = 1;

    let Some(listing) = posts_service::list_posts(&state.posts, first_page)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        return Ok(pagination_view::redirect("/posts/page/1"));
    };

    let pages_count: Vec<_> = (1..=listing.page.total).collect();

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"tt":&listing.total_items,"pages_count":pages_count,"tiger":listing.items,"o":all_category,"current_page":first_page}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn show_posts(
    path: web::Path<String>,
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.parse::<i32>().unwrap_or_default();

    let single_post_struct = posts_service::get_post(&state.posts, post_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = state
        .categories
        .all()
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

pub async fn get_category_posts(
    info: web::Path<(String, u32)>,
    state: web::Data<AppState>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (category_input, page_number) = info.into_inner();
    let category_id: i32 = category_input
        .parse()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let Some(listing) =
        posts_service::list_posts_for_category(&state.posts, category_id, page_number as i64)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
    else {
        let redirect_url = format!("/posts/category/{category_input}/page/1");
        return Ok(pagination_view::redirect(&redirect_url));
    };

    let pagination_final_string =
        general_category(listing.page.current, listing.page.total, &category_input);
    let pages_count: Vec<_> = (1..=listing.page.total).collect();

    let all_category = state
        .categories
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "category",
            &json!({"pagination":pagination_final_string,"tiger":&listing.items,"pages_count":&pages_count,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_redirect_user_integration() {
        let app =
            test::init_service(App::new().service(web::resource("/").to(redirect_user))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_redirection());
        let headers = resp.headers();
        assert_eq!(
            headers.get("location").unwrap().to_str().unwrap(),
            "/posts/page/1"
        );
    }
}
