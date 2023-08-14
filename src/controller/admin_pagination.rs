
pub async fn admin_pagination_with_category(
    cp: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {

    let x1 = r#"<div class="card mb-4">
                                <!-- Basic Pagination -->
                                   <!-- Basic Pagination -->
                                                <nav aria-label="Page navigation">
                                                    <ul class="pagination">
                                            "#;

let mut pagination_final_string = String::new();
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
Ok(pagination_final_string)
}


pub async fn admin_pagination_main_page(
    cp: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let mut pagination_final_string = String::new();


let test = r#"<div class="card mb-4">
                                <!-- Basic Pagination -->
                                   <!-- Basic Pagination -->
                                                <nav aria-label="Page navigation">
                                                    <ul class="pagination">
                                            "#;

pagination_final_string.push_str(test);
for i in 1..count_of_number_of_pages + 1 {
if i == cp {
let tag_and_url = r#"
             <li class="page-item active">
              <a class="page-link "   href="/admin/posts/page/"#;
pagination_final_string.push_str(tag_and_url);
let href_link = i.to_string();
pagination_final_string.push_str(&*href_link);
let end_of_tag = r#"">"#;
pagination_final_string.push_str(end_of_tag);
let text_inside_tag = i.to_string();
pagination_final_string.push_str(&*text_inside_tag);
let close_tag = r#"</a></li>"#;
pagination_final_string.push_str(close_tag);
} else {
let tag_and_url = r#"
             <li class="page-item">
              <a class="page-link "   href="/admin/posts/page/"#;
pagination_final_string.push_str(tag_and_url);
let href_link = i.to_string();
pagination_final_string.push_str(&*href_link);
let end_of_tag = r#"">"#;
pagination_final_string.push_str(end_of_tag);
let text_inside_tag = i.to_string();
pagination_final_string.push_str(&*text_inside_tag);
let close_tag = r#"</a></li>"#;
pagination_final_string.push_str(close_tag);
}
}
let v = r#"</ul>
        </nav>"#;
pagination_final_string.push_str(v);
Ok(pagination_final_string)
}