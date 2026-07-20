//! Post-related use cases: orchestrates ports + domain pagination rules.
//! Contains no HTTP, template, or SQL concerns.

use crate::application::ports::{CategoryRepository, PostRepository};
use crate::domain::category::Category;
use crate::domain::pagination::{resolve_page, Listing, DEFAULT_PAGE_SIZE};
use crate::domain::post::{NewPost, Post, PostWithCategory};
use anyhow::Result;

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
    repo.find(id).await
}

pub async fn create_post(repo: &impl PostRepository, new_post: &NewPost) -> Result<()> {
    if new_post.category_id == 0 {
        repo.create(new_post).await
    } else {
        repo.create_with_category(new_post, new_post.category_id)
            .await
    }
}

/// Updates a post, choosing the right persistence path depending on whether
/// it currently has a category and whether the submitted category changed.
pub async fn update_post(repo: &impl PostRepository, id: i32, updated: &NewPost) -> Result<()> {
    let current_category_id = repo.category_id_for_post(id).await.unwrap_or_default();

    if current_category_id == 0 {
        repo.attach_category(id, updated, updated.category_id).await
    } else if updated.category_id == 0 {
        repo.update_without_category(id, updated).await
    } else {
        repo.update(id, updated, updated.category_id).await
    }
}

pub async fn delete_post(repo: &impl PostRepository, id: i32) -> Result<()> {
    repo.delete(id).await
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
