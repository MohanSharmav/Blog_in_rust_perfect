pub async fn index_pagination(
    current_page: usize,
    total_pages_count: usize,
) -> Result<String, actix_web::Error> {
    // Html tags for pagination
    let start_tag = r#"
      <br>
      <div class="paginations">"#;
    let mut pagination_string = String::new();
    pagination_string.push_str(start_tag);
    // use for loop so that it will act as an array
    // example total_pages_count=3
    // use for loop and make [1,2,3]
    // and the check current_number with this total_pages_count array
    // start 1 because page number starts from 1
    for index in 1..total_pages_count + 1 {
        // if current page is equal to the array of total_pages_count item then it is marked with active tag
        // current page=2 and total_pages_count=3
        // if 2 = [1,2,3]
        // 2==2 then active
        if index == current_page {
            let tag_and_url = r#"<a class="active"  href="/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/page/"#;
            pagination_string.push_str(tag_and_url);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }
    }

    Ok(pagination_string)
}

pub async fn general_category(
    current_page: usize,
    total_pages_count: usize,
    category_id: &str,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::new();

    let end_tags = r#"
        <br>
        <div class="paginations">
        "#;
    pagination_string.push_str(end_tags);
    for index in 1..total_pages_count + 1 {
        if index == current_page {
            let tag_and_url = r#"<a class="active"  href="/posts/category/"#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_id.to_owned();
            pagination_string.push_str(&category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(static_keyword_page);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<a style="margin: 0 4px;" href="/posts/category/"#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_id.to_owned();
            pagination_string.push_str(&category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(static_keyword_page);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }
    }

    Ok(pagination_string)
}

pub async fn admin_posts_categories(
    current_page: usize,
    total_pages_count: usize,
    category_or_post: &str,
) -> Result<String, actix_web::Error> {
    let start_tag = r#"<div class="card mb-4">
                       <!-- Basic Pagination -->
                       <!-- Basic Pagination -->
                      <nav aria-label="Page navigation">
                       <ul class="pagination">
                        "#;
    let mut active_tag_and_url = String::new();
    let mut tag_and_url = String::new();
    if category_or_post == "post" {
        active_tag_and_url.push_str(
            r#"
             <li class="page-item active">
              <a class="page-link "   href="/admin/posts/page/"#,
        );
        tag_and_url.push_str(
            r#"
             <li class="page-item">
              <a class="page-link "   href="/admin/posts/page/"#,
        );
    } else {
        active_tag_and_url.push_str(
            r#"
              <li class="page-item active">
              <a class="page-link "   href="/admin/categories/page/"#,
        );
        tag_and_url.push_str(
            r#"
              <li class="page-item">
              <a class="page-link "   href="/admin/categories/page/"#,
        );
    }

    let mut pagination_string = String::new();
    pagination_string.push_str(start_tag);
    for index in 1..total_pages_count + 1 {
        if index == current_page {
            pagination_string.push_str(&*active_tag_and_url);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let page_constant = r#"">"#;
            pagination_string.push_str(page_constant);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            pagination_string.push_str(&*tag_and_url);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let page_constant = r#"">"#;
            pagination_string.push_str(page_constant);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        }
    }
    Ok(pagination_string)
}

pub async fn admin_category_posts(
    current_page: usize,
    total_pages_count: usize,
    category_id: String,
) -> Result<String, actix_web::Error> {
    let initial_tag = r#"
     <div class="card mb-4">
   <!-- Basic Pagination -->
   <!-- Basic Pagination -->
    <nav aria-label="Page navigation">
  <ul class="pagination">"#;
    let mut pagination_string = String::new();
    pagination_string.push_str(initial_tag);
    for index in 1..total_pages_count + 1 {
        if index == current_page {
            let tag_and_url = r#"
            <li class="page-item active">
              <a class="page-link "  href="/admin/categories/"#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_id.clone();
            pagination_string.push_str(&category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(static_keyword_page);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a>"#;
            pagination_string.push_str(close_tag);
        } else {
            let tag_and_url = r#"<li class="page-item">
              <a class="page-link "   href="/admin/categories/page/"
            "#;
            pagination_string.push_str(tag_and_url);
            let category_id = category_id.clone();
            pagination_string.push_str(&category_id);
            let static_keyword_page = r#"/page/"#;
            pagination_string.push_str(static_keyword_page);
            let href_link = index.to_string();
            pagination_string.push_str(&href_link);
            let end_of_tag = r#"">"#;
            pagination_string.push_str(end_of_tag);
            let text_inside_tag = index.to_string();
            pagination_string.push_str(&text_inside_tag);
            let close_tag = r#"</a> "#;
            pagination_string.push_str(close_tag);
        }
    }
    Ok(pagination_string)
}
