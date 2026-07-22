//! Black-box test of the server-rendered HTML surface — `blog-tests`'
//! `api_e2e.rs` only drives `/api/v1`, which never touches
//! `posts_admin.rs`/`categories_admin.rs`/`posts_guest.rs` at all. Uses raw
//! `reqwest` (not `blog-client`, which only speaks the JSON API) with
//! redirects disabled, so each hop can be asserted individually instead of
//! silently followed.
//!
//! This test is also what caught a real authorization gap: every mutating
//! admin HTML handler (`new_post`, `update_post`, `destroy_post`,
//! `create_category`, `update_category`, `destroy_category`) was missing the
//! `require_login` guard its sibling GET handlers had — anyone could create,
//! edit, or delete posts/categories through the HTML forms with no session
//! at all. Fixed alongside this test in `posts_admin.rs`/`categories_admin.rs`.

use blog_tests::TestServer;
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode};

fn http_client() -> Client {
    Client::builder()
        .cookie_store(true)
        .redirect(Policy::none())
        .build()
        .expect("build reqwest client")
}

fn is_html(resp: &reqwest::Response) -> bool {
    resp.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.contains("text/html"))
}

#[tokio::test]
async fn guest_pages_render_without_login() {
    let server = TestServer::spawn().await;
    let client = http_client();

    for path in [
        "/login",
        "/register",
        "/posts/page/1",
        "/posts/1",
        "/posts/category/1/page/1",
    ] {
        let resp = client
            .get(format!("{}{path}", server.base_url))
            .send()
            .await
            .unwrap_or_else(|e| panic!("GET {path}: {e}"));
        assert_eq!(resp.status(), StatusCode::OK, "GET {path}");
        assert!(is_html(&resp), "GET {path} should be text/html");
    }
}

#[tokio::test]
async fn admin_pages_redirect_to_login_when_not_authenticated() {
    let server = TestServer::spawn().await;
    let client = http_client();

    let get_paths = [
        "/admin/posts/page/1",
        "/admin/posts/new",
        "/admin/posts/1",
        "/admin/posts/1/edit",
        "/admin/post/1/delete",
        "/admin/categories/new",
        "/admin/categories/page/1",
        "/admin/category/1/edit",
        "/admin/category/1/delete",
        "/admin/categories/1/page/1",
    ];
    for path in get_paths {
        let resp = client
            .get(format!("{}{path}", server.base_url))
            .send()
            .await
            .unwrap_or_else(|e| panic!("GET {path}: {e}"));
        assert_eq!(
            resp.status(),
            StatusCode::SEE_OTHER,
            "GET {path} should redirect when logged out"
        );
        assert_eq!(resp.headers().get("location").unwrap(), "/", "GET {path}");
    }

    // The bug this test was written to catch: these used to perform the
    // mutation regardless of login. Confirm they now redirect instead.
    let post_cases: [(&str, &[(&str, &str)]); 4] = [
        (
            "/admin/posts",
            &[
                ("title", "should not be created"),
                ("description", "desc"),
                ("category_id", "0"),
            ],
        ),
        (
            "/admin/posts/1/edit",
            &[
                ("title", "should not overwrite seed data"),
                ("description", "desc"),
                ("category_id", "0"),
            ],
        ),
        (
            "/admin/categories/new",
            &[("name", "should not be created")],
        ),
        (
            "/admin/category/1/edit",
            &[("name", "should not rename seed data")],
        ),
    ];
    for (path, form) in post_cases {
        let resp = client
            .post(format!("{}{path}", server.base_url))
            .form(form)
            .send()
            .await
            .unwrap_or_else(|e| panic!("POST {path}: {e}"));
        assert_eq!(
            resp.status(),
            StatusCode::SEE_OTHER,
            "POST {path} should redirect when logged out, not perform the mutation"
        );
        assert_eq!(resp.headers().get("location").unwrap(), "/", "POST {path}");
    }

    // And confirm the mutations really didn't happen.
    let posts = client
        .get(format!("{}/posts/page/1", server.base_url))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(!posts.contains("should not overwrite seed data"));
}

#[tokio::test]
async fn authenticated_html_form_lifecycle() {
    let server = TestServer::spawn().await;
    let client = http_client();
    let base = &server.base_url;

    // Register and log in through the actual HTML forms (not the JSON API) —
    // `cookie_store(true)` on this client captures the session cookie from
    // `Set-Cookie` automatically and replays it on every later request.
    let resp = client
        .post(format!("{base}/register"))
        .form(&[("username", "html_user"), ("password", "hunter2")])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(resp.headers().get("location").unwrap(), "/login");

    let resp = client
        .post(format!("{base}/login"))
        .form(&[("username", "html_user"), ("password", "hunter2")])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get("location").unwrap(),
        "/admin/posts/page/1"
    );

    // Now-authenticated: admin pages serve content instead of redirecting.
    let resp = client
        .get(format!("{base}/admin/posts/page/1"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Create a category through the HTML form. Seed data has exactly 3
    // categories (ids 1-3), so on this fresh database the new one is
    // deterministically id 4.
    let resp = client
        .post(format!("{base}/admin/categories/new"))
        .form(&[("name", "html-created-category")])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get("location").unwrap(),
        "/admin/categories/page/1"
    );

    let edit_page = client
        .get(format!("{base}/admin/category/4/edit"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(edit_page.contains("html-created-category"));

    // Rename it through the HTML form.
    let resp = client
        .post(format!("{base}/admin/category/4/edit"))
        .form(&[("name", "renamed-category")])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let edit_page = client
        .get(format!("{base}/admin/category/4/edit"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(edit_page.contains("renamed-category"));
    assert!(!edit_page.contains("html-created-category"));

    // Create a post through the HTML form. Seed data has exactly 6 posts
    // (ids 1-6), so the new one is deterministically id 7.
    let resp = client
        .post(format!("{base}/admin/posts"))
        .form(&[
            ("title", "HTML-created post"),
            ("description", "written by html_e2e"),
            ("category_id", "4"),
        ])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let post_page = client
        .get(format!("{base}/admin/posts/7"))
        .send()
        .await
        .unwrap();
    assert_eq!(post_page.status(), StatusCode::OK);
    let post_page = post_page.text().await.unwrap();
    assert!(post_page.contains("HTML-created post"));

    // Edit it.
    let resp = client
        .post(format!("{base}/admin/posts/7/edit"))
        .form(&[
            ("title", "HTML-created post (edited)"),
            ("description", "still written by html_e2e"),
            ("category_id", "4"),
        ])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let post_page = client
        .get(format!("{base}/admin/posts/7"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(post_page.contains("HTML-created post (edited)"));

    // Delete it, then confirm.
    let resp = client
        .get(format!("{base}/admin/post/7/delete"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get("location").unwrap(),
        "/admin/posts/page/1"
    );

    let category_page = client
        .get(format!("{base}/admin/category/4/delete"))
        .send()
        .await
        .unwrap();
    assert_eq!(category_page.status(), StatusCode::SEE_OTHER);

    // Log out; admin pages redirect again.
    let resp = client.post(format!("{base}/logout")).send().await;
    // `/logout` is a bare GET-or-any resource in routes.rs (`.to(...)` with
    // no `.route()`), so POST works the same as GET here.
    assert!(resp.is_ok());

    let resp = client
        .get(format!("{base}/admin/posts/page/1"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
}
