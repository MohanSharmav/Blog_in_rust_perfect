//! In-memory fakes of the `blog-storage` ports, for unit-testing this
//! crate's services in milliseconds without a real database. `blog-storage`
//! itself is covered by its own DB-backed integration tests — these fakes
//! exist purely so the *branching logic* in `posts_service`/
//! `categories_service`/`auth_service` (which category-update path runs,
//! when a page resolves to `None`, ...) can be exercised directly.
#![cfg(test)]

use crate::ports::PasswordCipher;
use blog_storage::domain::category::Category;
use blog_storage::domain::post::{NewPost, Post, PostWithCategory};
use blog_storage::error::Result;
use blog_storage::ports::{CategoryRepository, PostRepository, UserRepository};
use std::sync::Mutex;

#[derive(Default)]
pub struct InMemoryPostRepository {
    posts: Mutex<Vec<Post>>,
    /// (post_id, category_id) — mirrors the `categories_posts` join table;
    /// a post with no row here has no category.
    links: Mutex<Vec<(i32, i32)>>,
    next_id: Mutex<i32>,
}

impl InMemoryPostRepository {
    pub fn new() -> Self {
        Self::default()
    }

    fn allocate_id(&self) -> i32 {
        let mut next = self.next_id.lock().unwrap();
        *next += 1;
        *next
    }
}

impl PostRepository for InMemoryPostRepository {
    async fn count(&self) -> Result<i64> {
        Ok(self.posts.lock().unwrap().len() as i64)
    }

    async fn count_for_category(&self, category_id: i32) -> Result<i64> {
        Ok(self
            .links
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, c)| *c == category_id)
            .count() as i64)
    }

    async fn page(&self, page_number: i32, per_page: i64) -> Result<Vec<Post>> {
        let start = (page_number as i64 - 1).max(0) as usize * per_page as usize;
        Ok(self
            .posts
            .lock()
            .unwrap()
            .iter()
            .skip(start)
            .take(per_page as usize)
            .cloned()
            .collect())
    }

    async fn page_for_category(
        &self,
        category_id: i32,
        page_number: i32,
        per_page: i64,
    ) -> Result<Vec<PostWithCategory>> {
        let posts = self.posts.lock().unwrap();
        let links = self.links.lock().unwrap();
        let start = (page_number as i64 - 1).max(0) as usize * per_page as usize;
        Ok(links
            .iter()
            .filter(|(_, c)| *c == category_id)
            .filter_map(|(post_id, _)| posts.iter().find(|p| p.id == *post_id))
            .skip(start)
            .take(per_page as usize)
            .map(|p| PostWithCategory {
                id: p.id,
                title: p.title.clone(),
                description: p.description.clone(),
                name: String::new(),
            })
            .collect())
    }

    async fn find(&self, id: i32) -> Result<Vec<Post>> {
        Ok(self
            .posts
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.id == id)
            .cloned()
            .collect())
    }

    async fn category_id_for_post(&self, post_id: i32) -> Result<i32> {
        Ok(self
            .links
            .lock()
            .unwrap()
            .iter()
            .find(|(p, _)| *p == post_id)
            .map(|(_, c)| *c)
            .unwrap_or_default())
    }

    async fn create(&self, new_post: &NewPost) -> Result<()> {
        let id = self.allocate_id();
        self.posts.lock().unwrap().push(Post {
            id,
            title: new_post.title.clone(),
            description: new_post.description.clone(),
        });
        Ok(())
    }

    async fn create_with_category(&self, new_post: &NewPost, category_id: i32) -> Result<()> {
        let id = self.allocate_id();
        self.posts.lock().unwrap().push(Post {
            id,
            title: new_post.title.clone(),
            description: new_post.description.clone(),
        });
        self.links.lock().unwrap().push((id, category_id));
        Ok(())
    }

    async fn update(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()> {
        self.update_fields(id, updated);
        self.links.lock().unwrap().retain(|(p, _)| *p != id);
        self.links.lock().unwrap().push((id, category_id));
        Ok(())
    }

    async fn update_without_category(&self, id: i32, updated: &NewPost) -> Result<()> {
        self.update_fields(id, updated);
        self.links.lock().unwrap().retain(|(p, _)| *p != id);
        Ok(())
    }

    async fn attach_category(&self, id: i32, updated: &NewPost, category_id: i32) -> Result<()> {
        self.update_fields(id, updated);
        self.links.lock().unwrap().push((id, category_id));
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        self.posts.lock().unwrap().retain(|p| p.id != id);
        self.links.lock().unwrap().retain(|(p, _)| *p != id);
        Ok(())
    }
}

impl InMemoryPostRepository {
    fn update_fields(&self, id: i32, updated: &NewPost) {
        if let Some(post) = self.posts.lock().unwrap().iter_mut().find(|p| p.id == id) {
            post.title = updated.title.clone();
            post.description = updated.description.clone();
        }
    }
}

#[derive(Default)]
pub struct InMemoryCategoryRepository {
    categories: Mutex<Vec<Category>>,
    next_id: Mutex<i32>,
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CategoryRepository for InMemoryCategoryRepository {
    async fn all(&self) -> Result<Vec<Category>> {
        Ok(self.categories.lock().unwrap().clone())
    }

    async fn all_except(&self, id: i32) -> Result<Vec<Category>> {
        Ok(self
            .categories
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.id != id)
            .cloned()
            .collect())
    }

    async fn page(&self, page_number: i32, per_page: i32) -> Result<Vec<Category>> {
        let start = (page_number as i64 - 1).max(0) as usize * per_page as usize;
        Ok(self
            .categories
            .lock()
            .unwrap()
            .iter()
            .skip(start)
            .take(per_page as usize)
            .cloned()
            .collect())
    }

    async fn find(&self, id: i32) -> Result<Vec<Category>> {
        Ok(self
            .categories
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.id == id)
            .cloned()
            .collect())
    }

    async fn count(&self) -> Result<i64> {
        Ok(self.categories.lock().unwrap().len() as i64)
    }

    async fn create(&self, name: &str) -> Result<()> {
        let mut next = self.next_id.lock().unwrap();
        *next += 1;
        self.categories.lock().unwrap().push(Category {
            id: *next,
            name: name.to_string(),
        });
        Ok(())
    }

    async fn update(&self, id: i32, name: &str) -> Result<()> {
        if let Some(category) = self
            .categories
            .lock()
            .unwrap()
            .iter_mut()
            .find(|c| c.id == id)
        {
            category.name = name.to_string();
        }
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        self.categories.lock().unwrap().retain(|c| c.id != id);
        Ok(())
    }
}

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: Mutex<Vec<(String, String)>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn register(&self, username: &str, password_hash: String) -> Result<()> {
        self.users
            .lock()
            .unwrap()
            .push((username.to_string(), password_hash));
        Ok(())
    }

    async fn credentials_match(&self, username: &str, password_hash: String) -> Result<bool> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .any(|(u, p)| u == username && *p == password_hash))
    }
}

/// A deterministic, insecure stand-in for `MagicCryptCipher` — real
/// encryption would work too, but this makes it obvious in a failing
/// assertion that the hash was never meant to be real ciphertext.
pub struct FakeCipher;

impl PasswordCipher for FakeCipher {
    fn encrypt(&self, plain: &str) -> String {
        format!("fake-hash({plain})")
    }
}
