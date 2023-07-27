use std::collections::HashMap;
use crate::controller::constants::ConfigurationConstants;
use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::model::category_database::get_all_categories_database;
use crate::model::database::{DataForFrontEnd, Pagination};
use crate::model::pagination_logic::select_specific_pages_post;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use actix_web_lab::sse::Data;
use ansi_term::Colour;
use handlebars::Handlebars;
use serde_json::json;

//
// pub async fn common_page_controller(
//     mut params: Option<Query<PaginationParams>>,
//     config: web::Data<ConfigurationConstants>,
//     handlebars: web::Data<Handlebars<'_>>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let db = &config.database_connection;
//     let total_posts_length = perfect_pagination_logic(db).await?;
//     let posts_per_page_constant = set_posts_per_page().await as i64;
//     let mut posts_per_page = total_posts_length / posts_per_page_constant;
//     let check_remainder = total_posts_length % posts_per_page_constant;
//     if check_remainder != 0 {
//         posts_per_page += 1;
//     }
//     let posts_per_page = posts_per_page as usize;
//     let pages_count: Vec<_> = (1..=posts_per_page).collect();
//     let pari = params.get_or_insert(Query(PaginationParams::default()));
//     let current_page = pari.clone().page;
//     let exact_posts_only = select_specific_pages_post(current_page, &db.clone())
//         .await
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     let all_category = get_all_categories_database(db)
//         .await
//         .map_err(actix_web::error::ErrorInternalServerError)?;
//
//     let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
//         .map_err( actix_web::error::ErrorInternalServerError)?;
//
//     Ok(HttpResponse::Ok()
//         .content_type(ContentType::html())
//         .body(htmls))
// }

pub async fn redirect_user() -> impl Responder {
    web::Redirect::to("/posts/page/1")
}

pub async fn set_posts_per_page() -> i32 {
    3
}

pub async fn new_common_page_controller(
    // mut params: Option<Query<PaginationParams>>,
    params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = perfect_pagination_logic(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    // let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let param = params.into_inner();
    let current_page = param as usize;

    let mut pages_count: Vec<_> = (1..=posts_per_page).collect();
    println!("---------------0----------------😀{:?}", pages_count);
    let mut pages_map = HashMap::new();

    // pages_count=Colour::Yellow.bold().paint(pages_count)
    let mut sample: Vec<_> = (1..=posts_per_page).collect();
    let mut c = 0;
    for i in sample.clone().into_iter() {
        if i == current_page {
            c = i;
            &pages_map.insert(100, c);
            sample.remove(i - 1);
            // sample.push(i);

            // sample.insert(i, i);

        }
        // else{
        //     &pages_map.insert(i,c);
        //
        // }

        // sample.remove(i);
    }
    println!("---------------0----------------👹{:?}", sample);
    let mut h =0;
 for  i in sample.clone().into_iter() {
     pages_map.insert(h, *&sample[h]);
h=h+1;
 }
    println!("--------------------------------🥵{:?}", c);
    for (key, value) in &pages_map {
        println!("------------Key: {}, Value: {}", key, value);
    }
    let final_pagination =Pagination{
        current_page:param ,
        other_pages: sample.clone(),
    };



    let serialized_person = serde_json::to_value(final_pagination.clone())?;

    // let template_str = "current_page: {{ current_page }}, other_pages: {{#each other_pages}}{{this}}, {{/each}}";


    let data = DataForFrontEnd {
        colored_text: "<span style=\"color: red;\">This text is red!</span>".to_string(),
    };

    // Create context to pass to the Handlebars template
    let context = serde_json::json!({
        "data": data,
    });



    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_page = pari.clone().page;
    // let par=*param as i32;
    let exact_posts_only = select_specific_pages_post(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"data":context,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"new_pagination":sample,"final_pagination":serialized_person,"page_map":pages_map}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn new_common_page_controller_test(
    // mut params: Option<Query<PaginationParams>>,
    params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = perfect_pagination_logic(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_page = pari.clone().page;
    let param = params.into_inner();
    // let par=*param as i32;
    let exact_posts_only = select_specific_pages_post(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common_two", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}

pub async fn Main_page(
    // mut params: Option<Query<PaginationParams>>,
    // params: web::Path<i32>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let total_posts_length = perfect_pagination_logic(db).await?;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let posts_per_page = posts_per_page as usize;
    // let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let param = 1;
    let current_page = param as usize;

    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    println!("---------------0----------------😀{:?}", pages_count);

    let mut sample: Vec<_> = (1..=posts_per_page).collect();
    let mut c = 0;
    for i in sample.clone().into_iter() {
        if i == current_page {
            c = i;
            sample.remove(i);
        }
    }
    println!("---------------0----------------👹{:?}", sample);

    println!("--------------------------------🥵{:?}", c);

    // let pari = params.get_or_insert(Query(PaginationParams::default()));
    // let current_page = pari.clone().page;
    // let par=*param as i32;
    let exact_posts_only = select_specific_pages_post(param, &db.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let htmls = handlebars.render("common", &json!({"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category,"current_page":param}))
        .map_err( actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(htmls))
}
