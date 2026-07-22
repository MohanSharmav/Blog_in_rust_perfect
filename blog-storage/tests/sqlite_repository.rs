#![cfg(feature = "sqlite")]
//! Integration test against a real, on-disk SQLite database: runs the actual
//! migrations, then exercises every repository through a full CRUD
//! round-trip. This is the thing CI runs to prove the SQLite backend works,
//! not just compiles.

use blog_storage::domain::post::NewPost;
use blog_storage::ports::{CategoryRepository, PostRepository, UserRepository};
use blog_storage::sqlite::{
    connect, SqliteCategoryRepository, SqlitePostRepository, SqliteUserRepository,
};
use std::sync::atomic::{AtomicU64, Ordering};

// A counter, not just a timestamp: tests run concurrently by default, and on
// some platforms `SystemTime::now()` doesn't actually have nanosecond
// resolution, so two tests starting in the same tick could otherwise compute
// the same "unique" file name and corrupt each other's data.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

async fn fresh_pool() -> sqlx::SqlitePool {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "blog-storage-test-{}-{id}.sqlite",
        std::process::id()
    ));
    let url = format!("sqlite://{}", path.display());

    let pool = connect(&url).await.expect("connect to sqlite");
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("run sqlite migrations");
    pool
}

#[tokio::test]
async fn seed_data_is_present_after_migration() {
    let pool = fresh_pool().await;
    let categories = SqliteCategoryRepository::new(pool.clone());
    let posts = SqlitePostRepository::new(pool);

    assert_eq!(categories.count().await.unwrap(), 3);
    assert_eq!(posts.count().await.unwrap(), 6);
}

#[tokio::test]
async fn category_crud_roundtrip() {
    let pool = fresh_pool().await;
    let categories = SqliteCategoryRepository::new(pool);

    categories.create("Roundtrip Category").await.unwrap();
    let all = categories.all().await.unwrap();
    let created = all
        .iter()
        .find(|c| c.name == "Roundtrip Category")
        .expect("created category should be listed");

    categories
        .update(created.id, "Renamed Category")
        .await
        .unwrap();
    let found = categories.find(created.id).await.unwrap();
    assert_eq!(found[0].name, "Renamed Category");

    categories.delete(created.id).await.unwrap();
    assert!(categories.find(created.id).await.unwrap().is_empty());
}

#[tokio::test]
async fn post_crud_roundtrip_with_category() {
    let pool = fresh_pool().await;
    let categories = SqliteCategoryRepository::new(pool.clone());
    let posts = SqlitePostRepository::new(pool);

    categories.create("Post Test Category").await.unwrap();
    let category_id = categories
        .all()
        .await
        .unwrap()
        .into_iter()
        .find(|c| c.name == "Post Test Category")
        .unwrap()
        .id;

    posts
        .create_with_category(
            &NewPost {
                title: "Roundtrip Post".to_string(),
                description: "Created in a test".to_string(),
                category_id,
            },
            category_id,
        )
        .await
        .unwrap();

    let page = posts.page(1, 100).await.unwrap();
    let created = page
        .iter()
        .find(|p| p.title == "Roundtrip Post")
        .expect("created post should appear in the page");
    let post_id = created.id;

    assert_eq!(
        posts.category_id_for_post(post_id).await.unwrap(),
        category_id
    );

    let category_page = posts.page_for_category(category_id, 1, 100).await.unwrap();
    assert!(category_page.iter().any(|p| p.id == post_id));

    posts
        .update_without_category(
            post_id,
            &NewPost {
                title: "Roundtrip Post Updated".to_string(),
                description: "Updated in a test".to_string(),
                category_id: 0,
            },
        )
        .await
        .unwrap();
    assert_eq!(posts.category_id_for_post(post_id).await.unwrap(), 0);

    let found = posts.find(post_id).await.unwrap();
    assert_eq!(found[0].title, "Roundtrip Post Updated");

    posts.delete(post_id).await.unwrap();
    assert!(posts.find(post_id).await.unwrap().is_empty());

    categories.delete(category_id).await.unwrap();
}

#[tokio::test]
async fn user_register_and_credentials_match() {
    let pool = fresh_pool().await;
    let users = SqliteUserRepository::new(pool);

    users
        .register("roundtrip_user", "hashed-password".to_string())
        .await
        .unwrap();

    assert!(users
        .credentials_match("roundtrip_user", "hashed-password".to_string())
        .await
        .unwrap());
    assert!(!users
        .credentials_match("roundtrip_user", "wrong-hash".to_string())
        .await
        .unwrap());
}
