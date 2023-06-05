use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::category_database::{category_controller_database_function, create_new_category_database, delete_category_database, get_all_categories_database, update_category_database};
use crate::model::database::categories;

pub async fn get_all_categories_controller()->HttpResponse {
    // println!("Checking categories");
    // println!("--------------------------------");
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/all_categories.hbs").unwrap();
    handlebars
        .register_template_string("all_categories", &index_template).expect("TODO: panic message");

     let all_categories = get_all_categories_database().await.expect("TODO: panic message");

    // println!(" 😋  😋  😋 {:?}",category_postinng);
    let html = handlebars.render("all_categories", &json!({"z":&all_categories})).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// pub async fn get_all_categories_controller()->HttpResponse{
//     println!("--------------------------------");
//     let mut handlebars= handlebars::Handlebars::new();
//     let index_template = fs::read_to_string("templates/all_categories.hbs").unwrap();
//     handlebars
//         .register_template_string("all_categories", &index_template).expect("TODO: panic message");
//
//     let all_categories=get_all_categories_database().await.expect("TODO: panic message");
//
//     // println!(" 😋  😋  😋 {:?}",category_postinng);
//     let html = handlebars.render("all_categories", &json!({"all_categories":&all_categories})).unwrap() ;
//
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }

pub async fn specific_category_controller(path: web::Path<String>) ->HttpResponse
{


    let mut category_input: String = path.into_inner();
    // println!("😊😊😊😊😊😊 comee on{:?}",category_input);
    // println!("😊😊😊😊😊😊-------------{:?}",category_input);
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template).expect("TODO: panic message");

    let category_postinng=category_controller_database_function(category_input).await.expect("TODO: panic message");

    println!(" 😋  😋  😋 {:?}",category_postinng);
    let html = handlebars.render("category", &json!({"p":&category_postinng})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


pub async fn get_new_category() -> HttpResponse {
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_category.hbs").unwrap();
    handlebars
        .register_template_string("new_category", &index_template).expect("TODO: panic message");

    println!("😇😇😇😇😇😇😇");
    let html = handlebars.render("new_category", &json!({"o":"ax"})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn receive_new_category(form: web::Form<categories>) -> HttpResponse
{
    println!("🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳");
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");


println!("🔥🔥🔥🔥🔥🔥🔥🔥");
    let name=&form.name;
    let id=&form.id;
    println!("------------------->{}", name);

    create_new_category_database(name,id).await.expect("TODO: panic message");
    let success_message="the categories created successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}

pub async fn delete_category(id: web::Path<String>) -> HttpResponse
{

    println!("ad🥳s🥳🥳🥳🥳🥳🥳🥳🥳🥳");
    let to_delete_category=&id.into_inner();
    println!("------->{}", to_delete_category);

    delete_category_database(to_delete_category).await.expect(" panic message");
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");
    let success_message="the category deleted successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}



pub async fn  page_to_update_category(to_be_updated_category: web::Path<String> )->HttpResponse{

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_category.hbs").unwrap();
    handlebars
        .register_template_string("update_category", &index_template).expect("TODO: panic message");

    let to_be_updated_category=to_be_updated_category.clone();
    println!("🤩🤩🤩🤩🤩{:?}", &to_be_updated_category);

//Todo should send the current post title to the next page
    let html = handlebars.render("update_category", &json!({"to_be_updated_post":&to_be_updated_category})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}

pub async fn receive_updated_category(form: web::Form<categories> , current_category_name: web::Path<String>) ->HttpResponse
{
    println!("-------------------------------------------------------------🇧🇾---");

    //todo get the data from the url form post method
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");
    // println!("----------------->{}",&web.into_inner());
    //Todo get the current post dynamically
    //  let current_post_name=&current_post_name.title;
    let current_post_name=&current_category_name.into_inner();

    let name=&form.name;
let id=&form.id;
    println!("🔥{:?}",current_post_name);
    println!("😇 new name is {:?}",name);
    // let to_be_updated_post= update_post_helper;
// println!("------------------------------>{}", title);

    update_category_database(&name,&current_post_name).await.expect("TODO: panic message");
    let success_message="the post created successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}