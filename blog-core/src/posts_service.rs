//! Post-related use cases: orchestrates ports + domain pagination rules.
//! Contains no HTTP, template, or SQL concerns.

use crate::error::Result;
use crate::pagination::{resolve_page, Listing, DEFAULT_PAGE_SIZE};
use blog_storage::domain::category::Category;
use blog_storage::domain::post::{NewPost, Post, PostWithCategory};
use blog_storage::ports::{CategoryRepository, PostRepository};

/// Lists posts for `page_number`. Returns `None` when the page is out of range.
pub async fn list_posts(
    repo: &impl PostRepository,
    page_number: i64,
) -> Result<Option<Listing<Post>>> {
    let total_items = repo.count().await?;
    let Some(page) = resolve_page(page_number, total_items, DEFAULT_PAGE_SIZE, false) else {
        return Ok(None);
    };
    let items = repo.page(page.current as i32, DEFAULT_PAGE_SIZE).await?;
    Ok(Some(Listing {
        items,
        page,
        total_items,
    }))
}

/// Lists posts within `category_id` for `page_number`. A category with no
/// posts still resolves to a single empty page rather than `None`, so
/// visiting its listing never redirect-loops.
pub async fn list_posts_for_category(
    repo: &impl PostRepository,
    category_id: i32,
    page_number: i64,
) -> Result<Option<Listing<PostWithCategory>>> {
    let total_items = repo.count_for_category(category_id).await?;
    let Some(page) = resolve_page(page_number, total_items, DEFAULT_PAGE_SIZE, true) else {
        return Ok(None);
    };
    let items = repo
        .page_for_category(category_id, page.current as i32, DEFAULT_PAGE_SIZE)
        .await?;
    Ok(Some(Listing {
        items,
        page,
        total_items,
    }))
}

pub async fn get_post(repo: &impl PostRepository, id: i32) -> Result<Vec<Post>> {
    Ok(repo.find(id).await?)
}

pub async fn create_post(repo: &impl PostRepository, new_post: &NewPost) -> Result<()> {
    if new_post.category_id == 0 {
        repo.create(new_post).await?;
    } else {
        repo.create_with_category(new_post, new_post.category_id)
            .await?;
    }
    Ok(())
}

/// Updates a post, choosing the right persistence path depending on whether
/// it currently has a category and whether the submitted category changed.
pub async fn update_post(repo: &impl PostRepository, id: i32, updated: &NewPost) -> Result<()> {
    let current_category_id = repo.category_id_for_post(id).await.unwrap_or_default();

    if current_category_id == 0 {
        repo.attach_category(id, updated, updated.category_id)
            .await?;
    } else if updated.category_id == 0 {
        repo.update_without_category(id, updated).await?;
    } else {
        repo.update(id, updated, updated.category_id).await?;
    }
    Ok(())
}

pub async fn delete_post(repo: &impl PostRepository, id: i32) -> Result<()> {
    Ok(repo.delete(id).await?)
}

/// Everything needed to render the "edit post" view.
pub struct PostEditView {
    pub post: Vec<Post>,
    pub category_info: Vec<Category>,
    pub other_categories: Vec<Category>,
}

pub async fn post_for_edit(
    posts: &impl PostRepository,
    categories: &impl CategoryRepository,
    post_id: i32,
) -> Result<PostEditView> {
    let post = posts.find(post_id).await?;
    let category_id = posts.category_id_for_post(post_id).await?;
    let category_info = categories.find(category_id).await?;
    let other_categories = categories.all_except(category_id).await?;

    Ok(PostEditView {
        post,
        category_info,
        other_categories,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fakes::InMemoryPostRepository;

    fn new_post(title: &str) -> NewPost {
        NewPost {
            title: title.to_string(),
            description: "a description".to_string(),
            category_id: 0,
        }
    }

    #[tokio::test]
    async fn list_posts_resolves_none_past_the_last_page() {
        let repo = InMemoryPostRepository::new();
        for i in 0..DEFAULT_PAGE_SIZE {
            repo.create(&new_post(&format!("post {i}"))).await.unwrap();
        }

        // Exactly one full page of DEFAULT_PAGE_SIZE items: page 1 exists, page 2 doesn't.
        assert!(list_posts(&repo, 1).await.unwrap().is_some());
        assert!(list_posts(&repo, 2).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn list_posts_for_category_returns_one_empty_page_when_category_has_no_posts() {
        let repo = InMemoryPostRepository::new();
        // No posts created at all: list_posts (clamp_min_one_page = false)
        // resolves to None, but list_posts_for_category (true) still
        // resolves a single empty page — see the doc comment on both.
        assert!(list_posts(&repo, 1).await.unwrap().is_none());

        let listing = list_posts_for_category(&repo, 42, 1)
            .await
            .unwrap()
            .expect("a category listing always has at least one page");
        assert!(listing.items.is_empty());
        assert_eq!(listing.page.total, 1);
    }

    #[tokio::test]
    async fn create_post_with_zero_category_id_creates_uncategorized() {
        let repo = InMemoryPostRepository::new();
        create_post(&repo, &new_post("uncategorized"))
            .await
            .unwrap();

        let posts = get_post(&repo, 1).await.unwrap();
        let post = posts.first().expect("post 1 should exist");
        assert_eq!(post.title, "uncategorized");
        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn create_post_with_category_id_attaches_it() {
        let repo = InMemoryPostRepository::new();
        let mut post = new_post("categorized");
        post.category_id = 7;
        create_post(&repo, &post).await.unwrap();

        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 7);
    }

    /// `update_post` picks one of three repository calls depending on
    /// whether the post currently has a category and whether the submitted
    /// category changed — these three tests are the actual point of having
    /// an in-memory fake: no other test in the workspace exercises this
    /// branching directly.
    #[tokio::test]
    async fn update_post_with_no_current_category_attaches_the_new_one() {
        let repo = InMemoryPostRepository::new();
        create_post(&repo, &new_post("start")).await.unwrap(); // category_id 0
        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 0);

        let mut updated = new_post("start, now categorized");
        updated.category_id = 3;
        update_post(&repo, 1, &updated).await.unwrap();

        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 3);
        assert_eq!(get_post(&repo, 1).await.unwrap()[0].title, updated.title);
    }

    #[tokio::test]
    async fn update_post_dropping_the_category_detaches_it() {
        let repo = InMemoryPostRepository::new();
        let mut initial = new_post("start");
        initial.category_id = 5;
        create_post(&repo, &initial).await.unwrap();
        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 5);

        let dropped = new_post("no longer categorized"); // category_id 0
        update_post(&repo, 1, &dropped).await.unwrap();

        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn update_post_changing_category_reattaches_it() {
        let repo = InMemoryPostRepository::new();
        let mut initial = new_post("start");
        initial.category_id = 5;
        create_post(&repo, &initial).await.unwrap();

        let mut moved = new_post("moved");
        moved.category_id = 9;
        update_post(&repo, 1, &moved).await.unwrap();

        assert_eq!(repo.category_id_for_post(1).await.unwrap(), 9);
    }

    #[tokio::test]
    async fn delete_post_removes_it_and_its_category_link() {
        let repo = InMemoryPostRepository::new();
        let mut post = new_post("to delete");
        post.category_id = 2;
        create_post(&repo, &post).await.unwrap();

        delete_post(&repo, 1).await.unwrap();

        assert!(get_post(&repo, 1).await.unwrap().is_empty());
        assert_eq!(repo.count_for_category(2).await.unwrap(), 0);
    }
}
