use crate::controller::common_controller::set_posts_per_page;
use crate::controller::constants::ConfigurationConstants;
use crate::model::category_database::{
    category_pagination_controller_database_function, get_all_categories_database,
};
use crate::model::pagination_database::{category_pagination_logic, pagination_logic};
use crate::model::single_posts_database::{query_single_post, query_single_post_in_struct};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{http, web, HttpResponse};
use askama::mime;
use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlPage};
use handlebars::Handlebars;
use serde_json::json;
use html_parser::Dom;
use warp::path::param;
use crate::controller::pagination_controller::perfect_pagination_logic;
use crate::model::pagination_logic::select_specific_pages_post;

pub async fn admin_category_display(
    // path: web::Path<String>,
    // params: web::Query<PaginationParams>,
    info: web::Path<(String, i32)>,
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
    let category_input: String = info.clone().0;
    let params = info.into_inner().1;
    // /**/let category_input: String = path.into_inner();
    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // let par=params.page;
    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let category_postinng = category_pagination_controller_database_function(
        category_input,
        db,
        params,
        posts_per_page,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_categories_page",
            &json!({"tiger":&category_postinng,"pages_count":&pages_count,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn admin_unique_posts_display(
    path: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = query_single_post_in_struct(titles, db)
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

pub async fn new_test(
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
let params = 4;
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
    // // let current_pag = pari.0;
    // let current_page = current_pag.page;
    let current_page = params.clone();
    let par = params.clone();
    let paginators = pagination_logic(&par, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let exact_posts_only = select_specific_pages_post(current_page, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;


    // let x2: String = HtmlPage::new()
    //     .with_title("My Page")
    //     .with_header(1, "Main Content:")
    //     .with_container(
    //         Container::new(ContainerType::Article)
    //             .with_attributes([("id", "article1")])
    //             .with_header_attr(2, "Hello, World", [("id", "article-head"), ("class", "header")])
    //             .with_paragraph("This is a simple HTML demo {{tiger}}")
    //     )
    //     .to_html_string();
    // let x= 1;

// println!("--------------------------------{:?}", html);
    let x1 = r#"
    <br>
<div class="paginations">
 "#;

    let y=pages_count.len();
// println!("--------------------------------😂{:?}",y);
// let cp=par
    let cp: usize = par as usize;
    // let mut act ="r#<a href="/">bosss</a>"#;
//     let  act = r#"
//     <br>
// <div class="paginations">
//   <a  >3</a>
//   <a class="active">1000</a>
//   <a >500</a>"#;
// // let x4=String::new();
   let mut x4 =String::new();
    x4.push_str(x1);
    for i in 1..y+1
    {
    // println!("--------------------------------😍{:?}",i);

    if i == cp
    {
        // let x2=r#"<a href="/">i</a>"#;
        //
        // println!("active{:?}",x2);
//       let on=  r#"<a href="/">bosss</a>"#;
//        let z= act.clone().to_string();
// act= &*(z.to_owned() + &on);

        // let x2=r#"<a href="/">one</a>"#;
        let x5=r#"<a class="active" href="/posts/page/"#;
        x4.push_str(x5);
        let x9=i.to_string();
        x4.push_str(&*x9);
        let x10=r#"">"#;
        x4.push_str(x10);
        let x6=i.to_string();
        x4.push_str(&*x6);

        let x7=r#"</a>"#;
        x4.push_str(x7);

        // let x2=x5.clone().to_owned()+ &*x6 +x7;
        // let x3=x1.clone().to_string();
         // x4= x1.to_owned() + &*x2.clone();
       // x4= x4.push_str(&*x2)

}
        else{
            // let x5=r#"<a href="/">"#;
            // x4.push_str(x5);
            // let x6=i.to_string();
            // x4.push_str(&*x6);
            //
            // let x7=r#"</a>"#;
            // x4.push_str(x7);
            let x5=r#"<a href="/posts/page/"#;
            x4.push_str(x5);
            let x9=i.to_string();
            x4.push_str(&*x9);
            let x10=r#"">"#;
            x4.push_str(x10);
            let x6=i.to_string();
            x4.push_str(&*x6);

            let x7=r#"</a>"#;
            x4.push_str(x7);
        }
//   {
//         // let x2=r#"<a href="/">i</a>"#;
//         //
//         // println!("not active{:?}",x2);
// // let x2=r#"<a href="/">one</a>"#;
//         let x5=r#"<a href="/">"#;
//         let x6=69798.to_string();
//         let x7=r#"</a>"#;
//         let x2=x5.clone().to_owned()+ &*x6 +x7;
//         let x3=x1.clone().to_string();
//         x4= x1.to_owned() + &*x2.clone();
//
//     }
}
//
//     let  x2 = r#"
//     <br>
// <div class="paginations">
//   <a  >3</a>
//   <a class="active">4</a>
//   <a >5</a>
//   <a >6</a>
// </div>"#;
// //
//     let x2=r#"
//     <br>
// <div class="paginations">
//   <a  >3</a>
//   <a class="active">4</a>
//   <a >5</a>
//   <a >6</a>
// </div>"#.to_string();
//     let x2=r#"<a href="/">one</a>"#;
//     let x3=x1.clone().to_string();
//     let x4=x1.to_owned()+ &*x2.clone();
// let fin=x4.to_owned()+x1;
    let html = handlebars
        .render(
            "sample",
            &json!({"x1":x4,"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count,"tiger":exact_posts_only,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
      )

}
