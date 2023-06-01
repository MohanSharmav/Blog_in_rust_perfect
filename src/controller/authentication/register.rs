use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;
use crate::controller::authentication::login::user;
use crate::model::authentication::register_database::register_new_user_database;

pub async fn get_register_page() -> HttpResponse {
    println!("Welcome to register page");
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/register.hbs").unwrap();
    handlebars
        .register_template_string("register", &index_template).expect("TODO: panic message");


    let html = handlebars.render("register", &json!({"yy":"uuihiuhuihiuhuih"})).unwrap();


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


pub async fn get_data_from_register_page(form: web::Form<user>) -> HttpResponse
{

    println!("🦋");

    let user = &form.username;
    let password=&form.password;

    println!("{}", user);

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");


    register_new_user_database(user, password).await.expect("TODO: panic message");

    let success_message="user successfully authenticated";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}