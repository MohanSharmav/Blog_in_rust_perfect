//! Ports: the contracts the application core depends on. Adapters (Postgres,
//! magic-crypt, ...) implement these traits; the core never depends on the
//! adapters themselves.

use crate::domain::category::Category;
use crate::domain::post::{NewPost, Post, PostWithCategory};
use anyhow::Result;

pub trait PostRepository {
    async fn count(&self) -> Result<i64>;
    async fn count_for_category(&self, category_id: i32) -> Result<i64>;
    async fn page(&self, page_number: i32, per_page: i64) -> Result<Vec<Post>>;
    async fn page_for_category(
        &self,
        category_id: i32,
        page_number: i32,
        per_page: i64,
    ) -> Result<Vec<PostWithCategory>>;
    async fn find(&self, id: i32) -> Result<Vec<Post>>;
    async fn category_id_for_post(&self, post_id: i32) -> Result<i32>;
    async fn create(&self, new_post: &NewPost) -> Result<()>;
    async fn create_with_category(&self, new_post: &NewPost, category_id: i32) -> Result<()>;
    async fn update(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()>;
    async fn update_without_category(&self, id: i32, updated: &NewPost) -> Result<()>;
    async fn attach_category(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
}

pub trait CategoryRepository {
    async fn all(&self) -> Result<Vec<Category>>;
    async fn all_except(&self, id: i32) -> Result<Vec<Category>>;
    async fn page(&self, page_number: i32, per_page: i32) -> Result<Vec<Category>>;
    async fn find(&self, id: i32) -> Result<Vec<Category>>;
    async fn count(&self) -> Result<i64>;
    async fn create(&self, name: &str) -> Result<()>;
    async fn update(&self, id: i32, name: &str) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
}

pub trait UserRepository {
    async fn register(&self, username: &str, password_hash: String) -> Result<()>;
    async fn credentials_match(&self, username: &str, password_hash: String) -> Result<bool>;
}

pub trait PasswordCipher {
    fn encrypt(&self, plain: &str) -> String;
}
