#![cfg(feature = "postgres")]
//! Integration test against a real Postgres database: runs the actual
//! migrations, then exercises every repository through a full CRUD
//! round-trip. Needs `DATABASE_URL` to point at a **scratch** Postgres
//! database (CI provides one via a service container); skips locally if
//! unset so `cargo test` doesn't fail for contributors without Postgres
//! running.
//!
//! Each test gets its own Postgres schema (`search_path` set via
//! `after_connect`, so it applies no matter which pooled connection serves a
//! given query) and runs migrations — including seed data — into that schema
//! fresh. That's what lets these run concurrently against one shared
//! `DATABASE_URL` without `--test-threads=1`: there's no shared table for two
//! tests to race on, each gets what's effectively its own database.

use blog_storage::domain::post::NewPost;
use blog_storage::ports::{CategoryRepository, PostRepository, UserRepository};
use blog_storage::postgres::{PgCategoryRepository, PgPostRepository, PgUserRepository};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Connects with a fresh, unique schema and runs migrations into it (or
/// returns `None` if `DATABASE_URL` isn't configured for this run).
async fn fresh_pool() -> Option<PgPool> {
    let database_url = std::env::var("DATABASE_URL").ok()?;

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let schema = format!("blog_test_{}_{id}", std::process::id());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .after_connect(move |conn, _meta| {
            let schema = schema.clone();
            Box::pin(async move {
                conn.execute(
                    format!(
                        r#"CREATE SCHEMA IF NOT EXISTS "{schema}"; SET search_path TO "{schema}";"#
                    )
                    .as_str(),
                )
                .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
        .expect("connect to postgres");

    sqlx::migrate!("./migrations/postgres")
        .run(&pool)
        .await
        .expect("run postgres migrations");
    Some(pool)
}

macro_rules! require_pool {
    () => {
        match fresh_pool().await {
            Some(pool) => pool,
            None => {
                eprintln!("skipping: DATABASE_URL not set");
                return;
            }
        }
    };
}

#[tokio::test]
async fn seed_data_is_present_after_migration() {
    let pool = require_pool!();
    let categories = PgCategoryRepository::new(pool.clone());
    let posts = PgPostRepository::new(pool);

    assert_eq!(categories.count().await.unwrap(), 3);
    assert_eq!(posts.count().await.unwrap(), 6);
}

#[tokio::test]
async fn category_crud_roundtrip() {
    let pool = require_pool!();
    let categories = PgCategoryRepository::new(pool);

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
    let pool = require_pool!();
    let categories = PgCategoryRepository::new(pool.clone());
    let posts = PgPostRepository::new(pool);

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
    let pool = require_pool!();
    let users = PgUserRepository::new(pool);

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
