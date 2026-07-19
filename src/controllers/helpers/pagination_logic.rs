use actix_http::header::LOCATION;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use std::fmt::Write;

/// Total number of pages needed to show `total_items` at `per_page` items per page.
pub fn total_pages(total_items: i64, per_page: i64) -> i64 {
    let pages = total_items / per_page;
    if total_items % per_page != 0 {
        pages + 1
    } else {
        pages
    }
}

/// Validates a 1-indexed `page_param` against the item/page-size counts.
///
/// Returns the resolved `(current_page, total_pages)` on success, or a
/// ready-to-return redirect to `redirect_url` if the page is out of range.
/// When `clamp_min_one_page` is set, a listing with zero items still counts
/// as having one (empty) page rather than redirecting forever.
pub fn resolve_current_page(
    page_param: i64,
    total_items: i64,
    per_page: i64,
    clamp_min_one_page: bool,
    redirect_url: &str,
) -> Result<(usize, usize), HttpResponse> {
    let mut pages = total_pages(total_items, per_page) as usize;
    if clamp_min_one_page {
        pages = pages.max(1);
    }
    let current_page = page_param as usize;

    if current_page == 0 || current_page > pages {
        Err(HttpResponse::SeeOther()
            .insert_header((LOCATION, redirect_url.to_owned()))
            .content_type(ContentType::html())
            .finish())
    } else {
        Ok((current_page, pages))
    }
}

pub async fn index_pagination(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::from(
        r#"
      <br>
      <div class="paginations">"#,
    );

    if count_of_number_of_pages == 0 {
        pagination_string.push_str(r#"<a class="active"  href="/posts/page/1">1</a>"#);
    }

    for i in 1..=count_of_number_of_pages {
        if i == current_page {
            let _ = write!(
                pagination_string,
                r#"<a class="active"  href="/posts/page/{i}">{i}</a>"#
            );
        } else {
            let _ = write!(
                pagination_string,
                r#"<a style="margin: 0 4px;" href="/posts/page/{i}">{i}</a>"#
            );
        }
    }

    Ok(pagination_string)
}

pub async fn general_category(
    current_page: usize,
    count_of_number_of_pages: usize,
    category_input: &str,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::from(
        r#"
        <br>
        <div class="paginations">
        "#,
    );

    if count_of_number_of_pages == 0 {
        pagination_string.push_str(r#"<a class="active"  href="/posts/page/1">1</a>"#);
    }

    for i in 1..=count_of_number_of_pages {
        if i == current_page {
            let _ = write!(
                pagination_string,
                r#"<a class="active"  href="/posts/category/{category_input}/page/{i}">{i}</a>"#
            );
        } else {
            let _ = write!(
                pagination_string,
                r#"<a style="margin: 0 4px;" href="/posts/category/{category_input}/page/{i}">{i}</a>"#
            );
        }
    }

    Ok(pagination_string)
}

pub async fn admin_categories(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::from(
        r#"<div class="card mb-4">
                       <!-- Basic Pagination -->
                       <!-- Basic Pagination -->
                      <nav aria-label="Page navigation">
                       <ul class="pagination">
                        "#,
    );

    for i in 1..=count_of_number_of_pages {
        let active_class = if i == current_page { " active" } else { "" };
        let _ = write!(
            pagination_string,
            r#"
              <li class="page-item{active_class}">
              <a class="page-link "   href="/admin/categories/page/{i}">{i}</a>"#
        );
    }
    Ok(pagination_string)
}

pub async fn admin_main_page(
    current_page: usize,
    count_of_number_of_pages: usize,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::from(
        r#"<div class="card mb-4">
                        <!-- Basic Pagination -->
                        <!-- Basic Pagination -->
                        <nav aria-label="Page navigation">
                        <ul class="pagination">
                        "#,
    );

    for i in 1..=count_of_number_of_pages {
        let active_class = if i == current_page { " active" } else { "" };
        let _ = write!(
            pagination_string,
            r#"
             <li class="page-item{active_class}">
              <a class="page-link "   href="/admin/posts/page/{i}">{i}</a></li>"#
        );
    }

    pagination_string.push_str(
        r#"</ul>
        </nav>"#,
    );
    Ok(pagination_string)
}

pub async fn admin_category_posts(
    current_page: usize,
    count_of_number_of_pages: usize,
    category_input: String,
) -> Result<String, actix_web::Error> {
    let mut pagination_string = String::from(
        r#"
     <div class="card mb-4">
   <!-- Basic Pagination -->
   <!-- Basic Pagination -->
    <nav aria-label="Page navigation">
  <ul class="pagination">"#,
    );

    for i in 1..=count_of_number_of_pages {
        if i == current_page {
            let _ = write!(
                pagination_string,
                r#"
            <li class="page-item active">
              <a class="page-link "  href="/admin/categories/{category_input}/page/{i}">{i}</a>"#
            );
        } else {
            let _ = write!(
                pagination_string,
                r#"<li class="page-item">
              <a class="page-link "   href="/admin/categories/{category_input}/page/{i}">{i}</a> "#
            );
        }
    }
    Ok(pagination_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_index_pagination_zero_pages() {
        let result = index_pagination(1, 0).await.unwrap();
        assert!(result.contains(r#"<a class="active"  href="/posts/page/1">1</a>"#));
    }

    #[actix_web::test]
    async fn test_index_pagination_multiple_pages() {
        let result = index_pagination(2, 3).await.unwrap();
        assert!(result.contains(r#"<a style="margin: 0 4px;" href="/posts/page/1">1</a>"#));
        assert!(result.contains(r#"<a class="active"  href="/posts/page/2">2</a>"#));
        assert!(result.contains(r#"<a style="margin: 0 4px;" href="/posts/page/3">3</a>"#));
    }

    #[actix_web::test]
    async fn test_admin_categories_pagination() {
        let result = admin_categories(2, 3).await.unwrap();
        assert!(result.contains(r#"<a class="page-link "   href="/admin/categories/page/1">1</a>"#));
        assert!(result.contains(r#"<li class="page-item active">"#));
        assert!(result.contains(r#"<a class="page-link "   href="/admin/categories/page/2">2</a>"#));
    }

    #[actix_web::test]
    async fn test_general_category_links_include_category() {
        let result = general_category(1, 2, "rust").await.unwrap();
        assert!(result.contains(r#"href="/posts/category/rust/page/1""#));
        assert!(result.contains(r#"href="/posts/category/rust/page/2""#));
    }

    #[actix_web::test]
    async fn test_admin_category_posts_non_active_links_are_well_formed() {
        // Regression test: the non-active branch used to close the href
        // attribute early, producing a link that pointed at
        // "/admin/categories/page/" (missing the category and page number)
        // instead of "/admin/categories/{category}/page/{n}".
        let result = admin_category_posts(1, 2, "rust-lang".to_string())
            .await
            .unwrap();
        assert!(result.contains(r#"href="/admin/categories/rust-lang/page/2""#));
    }
}
