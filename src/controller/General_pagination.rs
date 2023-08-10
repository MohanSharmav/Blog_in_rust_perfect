use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use actix_web::http::header::ContentType;
use handlebars::Handlebars;
use serde_json::json;
use crate::controller::constants::ConfigurationConstants;

pub async fn general_pagination(
    cp: usize,
    count_of_number_of_pages: usize

) -> Result<String, actix_web::Error>
{


    let x1 = r#"
    <br>
<div class="paginations">
 "#;

    let mut pagination_final_string = String::new();
    pagination_final_string.push_str(x1);

    if count_of_number_of_pages==0{
        let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
        pagination_final_string.push_str(tag_and_url);
        let href_link = 1.to_string();
        pagination_final_string.push_str(&*href_link);
        let end_of_tag = r#"">"#;
        pagination_final_string.push_str(end_of_tag);
        let text_inside_tag = 1.to_string();
        pagination_final_string.push_str(&*text_inside_tag);
        let close_tag = r#"</a>"#;
        pagination_final_string.push_str(close_tag);
    }

    for i in 1..count_of_number_of_pages + 1 {
         if i == cp {
            let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
            pagination_final_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_final_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_final_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_final_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_final_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/page/"#;
            pagination_final_string.push_str(tag_and_url);
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

    Ok(pagination_final_string)
}


pub async fn general_pagination_with_category(
    cp: usize,
    count_of_number_of_pages: usize,
    category_input: &String,
    admin: bool
) -> Result<String, actix_web::Error>{

    let mut pagination_final_string = String::new();

    if admin
    {
        let x1 = r#"<div class="card mb-4">
                                <!-- Basic Pagination -->
                                   <!-- Basic Pagination -->
                                                <nav aria-label="Page navigation">
                                                    <ul class="pagination">
                                            "#;

        pagination_final_string.push_str(x1);
        for i in 1..count_of_number_of_pages + 1 {
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

    }else{
    let x1 = r#"
    <br>
<div class="paginations">
 "#;

    pagination_final_string.push_str(x1);

    if count_of_number_of_pages==0{
        let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
        pagination_final_string.push_str(tag_and_url);
        let href_link = 1.to_string();
        pagination_final_string.push_str(&*href_link);
        let end_of_tag = r#"">"#;
        pagination_final_string.push_str(end_of_tag);
        let text_inside_tag = 1.to_string();
        pagination_final_string.push_str(&*text_inside_tag);
        let close_tag = r#"</a>"#;
        pagination_final_string.push_str(close_tag);
    }

    for i in 1..count_of_number_of_pages + 1 {
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
    }
    Ok(pagination_final_string)

}