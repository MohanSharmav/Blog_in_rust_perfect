pub async fn index_pagination(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let start_tag = r#"
      <br>
      <div class="paginations">"#;

    let mut pagination_string = String::new();
    pagination_string.push_str(start_tag);

    if count_of_number_of_pages == 0 {
        let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
        pagination_string.push_str(tag_and_url);
        let href_link = 1.to_string();
        pagination_string.push_str(&*href_link);
        let end_of_tag = r#"">"#;
        pagination_string.push_str(end_of_tag);
        let text_inside_tag = 1.to_string();
        pagination_string.push_str(&*text_inside_tag);
        let close_tag = r#"</a>"#;
        pagination_string.push_str(close_tag);
    }

    for i in 1..count_of_number_of_pages + 1 {
        if i == current_page {
            let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }
    }

    Ok(pagination_string)
}

pub async fn general_category(
    current_page: usize,
    count_of_number_of_pages: usize,
    category_input: &String,
    admin: bool,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::new();

    if admin {
        let starting_tag = r#"<div class="card mb-4">
             <!-- Basic Pagination -->
              <!-- Basic Pagination -->
             <nav aria-label="Page navigation">
              <ul class="pagination">"#;

        pagination_string.push_str(starting_tag);
        for i in 1..count_of_number_of_pages + 1 {
            if i == current_page {
                let tag_and_url = r#"
               <li class="page-item active">
               <a class="page-link "   href="/admin/categories/page/"#;
                pagination_string.push_str(tag_and_url);
                let href_link = i.to_string();
                pagination_string.push_str(&*href_link);
                let page_constant = r#"">"#;
                pagination_string.push_str(page_constant);
                let text_inside_tag = i.to_string();
                pagination_string.push_str(&*text_inside_tag);
                let close_tag = r#"</a>"#;
                pagination_string.push_str(close_tag);
            } else {
                let tag_and_url = r#"
               <li class="page-item">
               <a class="page-link "   href="/admin/categories/page/"#;
                pagination_string.push_str(tag_and_url);
                let href_link = i.to_string();
                pagination_string.push_str(&*href_link);
                let page_constant = r#"">"#;
                pagination_string.push_str(page_constant);
                let text_inside_tag = i.to_string();
                pagination_string.push_str(&*text_inside_tag);
                let close_tag = r#"</a>"#;
                pagination_string.push_str(close_tag);
            }
        }
    } else {
        let end_tags = r#"
        <br>
        <div class="paginations">
        "#;

        pagination_string.push_str(end_tags);

        if count_of_number_of_pages == 0 {
            let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = 1.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = 1.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }

        for i in 1..count_of_number_of_pages + 1 {
            if i == current_page {
                let tag_and_url = r#"<a class="active"  href="/posts/category/"#;
                pagination_string.push_str(tag_and_url);
                let category_id = category_input.clone();
                pagination_string.push_str(&*category_id);
                let static_keyword_page = r#"/page/"#;
                pagination_string.push_str(&*static_keyword_page);
                let href_link = i.to_string();
                pagination_string.push_str(&*href_link);
                let end_of_tag = r#"">"#;
                pagination_string.push_str(end_of_tag);
                let text_inside_tag = i.to_string();
                pagination_string.push_str(&*text_inside_tag);
                let close_tag = r#"</a>"#;
                pagination_string.push_str(close_tag);
            } else {
                let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/category/"#;
                pagination_string.push_str(tag_and_url);
                let category_id = category_input.clone();
                pagination_string.push_str(&*category_id);
                let static_keyword_page = r#"/page/"#;
                pagination_string.push_str(&*static_keyword_page);
                let href_link = i.to_string();
                pagination_string.push_str(&*href_link);
                let end_of_tag = r#"">"#;
                pagination_string.push_str(end_of_tag);
                let text_inside_tag = i.to_string();
                pagination_string.push_str(&*text_inside_tag);
                let close_tag = r#"</a>"#;
                pagination_string.push_str(close_tag);
            }
        }
    }
    Ok(pagination_string)
}

pub async fn admin_categories(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let start_tag = r#"<div class="card mb-4">
                       <!-- Basic Pagination -->
                       <!-- Basic Pagination -->                                   <nav aria-label="Page navigation">
                       <ul class="pagination">
                                            "#;

    let mut pagination_string = String::new();
    pagination_string.push_str(start_tag);
    for i in 1..count_of_number_of_pages + 1 {
        if i == current_page {
            let tag_and_url = r#"
              <li class="page-item active">
              <a class="page-link "   href="/admin/categories/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let page_constant = r#"">"#;
            pagination_string.push_str(page_constant);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"
              <li class="page-item">
              <a class="page-link "   href="/admin/categories/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let page_constant = r#"">"#;
            pagination_string.push_str(page_constant);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }
    }
    Ok(pagination_string)
}

pub async fn admin_main_page(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::new();

    let start_tag = r#"<div class="card mb-4">
                        <!-- Basic Pagination -->
                        <!-- Basic Pagination -->
                        <nav aria-label="Page navigation">
                        <ul class="pagination">
                        "#;

    pagination_string.push_str(start_tag);
    for i in 1..count_of_number_of_pages + 1 {
        if i == current_page {
            let tag_and_url = r#"
             <li class="page-item active">
              <a class="page-link "   href="/admin/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a></li>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"
             <li class="page-item">
              <a class="page-link "   href="/admin/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a></li>"#;
            pagination_string.push_str(close_tag);
        }
    }
    let end_tag = r#"</ul>
        </nav>"#;
    pagination_string.push_str(end_tag);
    Ok(pagination_string)
}

pub async fn admin_category_posts(
    current_page: usize,
    count_of_number_of_pages: usize,
    category_input: String,
) -> Result<String, actix_web::Error> {
    let initial_tag = r#"
     <div class="card mb-4">
   <!-- Basic Pagination -->
   <!-- Basic Pagination -->
    <nav aria-label="Page navigation">
  <ul class="pagination">"#;

    let mut pagination_string = String::new();
    pagination_string.push_str(initial_tag);
    for i in 1..count_of_number_of_pages + 1 {
        if i == current_page {
            let tag_and_url = r#"
            <li class="page-item active">
              <a class="page-link "  href="/admin/categories/"#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_input.clone();
            pagination_string.push_str(&*category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(&*static_keyword_page);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<li class="page-item">
              <a class="page-link "   href="/admin/categories/page/"
            "#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_input.clone();
            pagination_string.push_str(&*category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(&*static_keyword_page);
            let href_link = i.to_string();
            pagination_string.push_str(&*href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = i.to_string();
            pagination_string.push_str(&*text_inside_tag);
            let close_tag = r#"</a> "#;
            pagination_string.push_str(close_tag);
        }
    }
    Ok(pagination_string)
}
