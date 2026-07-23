//! Black-box test of the `/api/v1` JSON API: spawns a real `blog-server`
//! against a scratch database and drives it entirely through `blog-client`,
//! exactly as a real caller would — no direct access to the server's
//! internals.

use blog_client::{BlogClient, Category, ClientError, NewPost, Post};
use blog_tests::TestServer;

/// Categories/posts are paginated (3 per page) and the seed data already
/// fills page 1, so a newly created item can land on a later page — search
/// every page rather than assuming page 1.
async fn find_category(client: &BlogClient, name: &str) -> Category {
    let mut page = 1;
    loop {
        let listing = client.list_categories(page).await.expect("list categories");
        if let Some(found) = listing.items.into_iter().find(|c| c.name == name) {
            return found;
        }
        assert!(
            page < listing.total_pages as i64,
            "category {name:?} not found in any page"
        );
        page += 1;
    }
}

async fn find_post(client: &BlogClient, title: &str) -> Post {
    let mut page = 1;
    loop {
        let listing = client.list_posts(page).await.expect("list posts");
        if let Some(found) = listing.items.into_iter().find(|p| p.title == title) {
            return found;
        }
        assert!(
            page < listing.total_pages as i64,
            "post {title:?} not found in any page"
        );
        page += 1;
    }
}

#[tokio::test]
async fn full_auth_and_content_lifecycle() {
    let server = TestServer::spawn().await;
    let client = BlogClient::new(&server.base_url);

    // Unauthenticated calls are rejected.
    let err = client
        .create_category("should not be allowed")
        .await
        .expect_err("anonymous create_category should fail");
    assert!(matches!(err, ClientError::NotAuthenticated));

    // Register + login.
    client.register("alice", "hunter2").await.expect("register");
    client.login("alice", "hunter2").await.expect("login");
    assert_eq!(client.me().await.expect("me"), "alice");

    // Wrong password is rejected distinctly from "not logged in".
    let anon = BlogClient::new(&server.base_url);
    let err = anon
        .login("alice", "wrong-password")
        .await
        .expect_err("bad password should fail");
    assert!(matches!(err, ClientError::InvalidCredentials));

    // Category CRUD.
    client
        .create_category("rust")
        .await
        .expect("create category");
    let category_id = find_category(&client, "rust").await.id;

    client
        .update_category(category_id, "rust-lang")
        .await
        .expect("update category");
    let renamed = client
        .get_category(category_id)
        .await
        .expect("get category");
    assert_eq!(renamed.name, "rust-lang");

    // Post CRUD, attached to that category.
    client
        .create_post(&NewPost {
            title: "Hello, blog-tests".to_string(),
            description: "written by the e2e suite".to_string(),
            category_id,
        })
        .await
        .expect("create post");

    let post_id = find_post(&client, "Hello, blog-tests").await.id;

    client
        .update_post(
            post_id,
            &NewPost {
                title: "Hello, blog-tests (edited)".to_string(),
                description: "still written by the e2e suite".to_string(),
                category_id,
            },
        )
        .await
        .expect("update post");
    let updated = client.get_post(post_id).await.expect("get post");
    assert_eq!(updated.title, "Hello, blog-tests (edited)");

    client.delete_post(post_id).await.expect("delete post");
    let err = client
        .get_post(post_id)
        .await
        .expect_err("deleted post should 404");
    assert!(matches!(err, ClientError::NotFound));

    client
        .delete_category(category_id)
        .await
        .expect("delete category");

    // Logout invalidates the session.
    client.logout().await.expect("logout");
    let err = client.me().await.expect_err("me after logout should fail");
    assert!(matches!(err, ClientError::NotAuthenticated));
}
