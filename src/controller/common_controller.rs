use std::mem;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::category_database::get_all_categories_database;
use crate::model::pagination_database::PaginationParams;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use std::option::Option;
use actix_web::web::{Query, Redirect};
use handlebars::Handlebars;
use serde_json::json;

pub async fn common_page_controller(
    params: Option<Query<PaginationParams>>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    // let x= PaginationParams{ page: 0 };
    // if params==actix_web::web::Query(x)
    // let x=params.clone().into_inner().to_string();
    // if x==""
    // {
    //     println!("😀");
    //     web::Redirect::to("/?page=1");
    // }
    // let params=1;

    println!("--------------------------------🙃{}",params.is_some());
 // let  params=PaginationParams::default();


   let mut par = match params{
        Some(_)=>params.map(|i|i),
    None=>Some(Query::<PaginationParams>::from(actix_web::web::Query(PaginationParams::default())))
    };
    // if mem::size_of::<PaginationParams>() == 0 {
    //  PaginationParams::default();
    // }

        let db = &config.database_connection;
    let total_posts_length: f64 = perfect_pagination_logic(db).await? as f64;
    let posts_per_page = total_posts_length / 3.0;
    let posts_per_page = posts_per_page.round();
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let  pari=par.get_or_insert(Query(PaginationParams::default()));
    let current_page = pari.clone().page;
   // let v= par.get("PaginationParams").unwrap_or(&String::from("x"));
   //   let current_page:i32 = v.page?;

    let exact_posts_only = select_specific_pages_post(current_page, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn home_redirect() -> Redirect {
    web::Redirect::to("/?page=1")
}