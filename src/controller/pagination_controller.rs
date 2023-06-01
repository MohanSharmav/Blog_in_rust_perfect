use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Row};
use warp::path;
use crate::controller::pagination_logic::select_specific_pages_post;
use crate::model::database::{posts, select_posts, get_all_categories};
use crate::model::pagination_database::{ pagination_logic, PaginationParams};
use crate::model::Single_posts_database::query_single_post;


pub async  fn  get_count_of_posts () -> HttpResponse {
    // println!("{:?}",x);
    // let v=x as i32;
    // //  static mut n: i32 = v;
    // let total_pages_count= x  as i32;
    // println!("😊😊😊😊😊😊{:?}",total_pages_count);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT title,description,name FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();



    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_string= title+" " +&*description +" "+ &*name;
        // let all_posts_string=format!(title, description, name);
        //   let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
        all_posts.push(all_posts_string);
    }

    let total_posts_count:i32 = all_posts.len() as i32;

    println!("Total posts: {}", total_posts_count);


    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");

    // let all_categories= get_all_categories().await.expect("adssad");
// println!("------{:?}", all_categories);
    // path: web::Path<String>
   // let single_post=query_single_post(titles).await.expect("TODO: panic message");

    println!("😊😊😊😊😊😊ijhijijijij 😊😊😊😊😊😊 {:?}",total_posts_count);
    let html = handlebars.render("pagination_page", &json!({"bb":&total_posts_count,"yy":"uuihiuhuihiuhuih"})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}
//
pub async fn pagination_display( params: web::Query<PaginationParams> ) ->HttpResponse{


  let mut total_posts_length:f64= perfect_pagination_logic().await as f64;


  let  posts_per_page=total_posts_length/3.0;


    let posts_per_page=posts_per_page.round();
    let posts_per_page=posts_per_page as i64;
    let mut pages_count=Vec::new();
    for i in 0..posts_per_page{
     pages_count.push(i+1 as i64);
    }

    println!("pagesss count{:?}", pages_count);


    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");


    let paginators= pagination_logic(params.clone()).await.expect("Aasd");


  //  println!("😊😊😊😊😊{:?}", exact_posts_only);
    // let pagination_count:i32= get_count_of_posts().await;
// todo call the exact_posts_only function with out parameter or find other way
    let current_page=&params.page;
     let exact_posts_only=select_specific_pages_post(current_page).await.expect("Aasd");

    let all_category= get_all_categories().await.expect("adssad");

//    println!("s😊😊😊😊😊😊{:?}", pagination_count);
println!("🐍{:?}",exact_posts_only);

   let html = handlebars.render("pagination_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;

    let mut handlebarss= handlebars::Handlebars::new();
    let index_templates = fs::read_to_string("templates/admin_page.hbs").unwrap();
    handlebarss
        .register_template_string("admin_page", &index_templates).expect("TODO: panic message");

    let htmls = handlebarss.render("admin_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(htmls)

}

pub async fn perfect_pagination_logic() -> i64 {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");



    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();


let mut counting_final:i64= 0;
    for row in rows{
        let title:i64 = row.try_get("count").unwrap();;
        counting_final=counting_final+title;
        println!("{:?}", title);
    }
counting_final
}