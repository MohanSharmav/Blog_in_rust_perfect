//! Category-related use cases.

use crate::application::ports::CategoryRepository;
use crate::domain::category::Category;
use crate::domain::pagination::{resolve_page, Listing, DEFAULT_PAGE_SIZE};
use anyhow::Result;

/// Lists categories for `page_number`. Returns `None` when the page is out of range.
pub async fn list_categories(
    repo: &impl CategoryRepository,
    page_number: i64,
) -> Result<Option<Listing<Category>>> {
    let total_items = repo.count().await?;
    let Some(page) = resolve_page(page_number, total_items, DEFAULT_PAGE_SIZE, false) else {
        return Ok(None);
    };
    let items = repo
        .page(page.current as i32, DEFAULT_PAGE_SIZE as i32)
        .await?;
    Ok(Some(Listing {
        items,
        page,
        total_items,
    }))
}

pub async fn create_category(repo: &impl CategoryRepository, name: &str) -> Result<()> {
    repo.create(name).await
}

pub async fn update_category(repo: &impl CategoryRepository, id: i32, name: &str) -> Result<()> {
    repo.update(id, name).await
}

pub async fn delete_category(repo: &impl CategoryRepository, id: i32) -> Result<()> {
    repo.delete(id).await
}

/// Everything needed to render the "edit category" view.
pub struct CategoryEditView {
    pub all_categories: Vec<Category>,
    pub current_name: Vec<Category>,
}

pub async fn category_for_edit(
    repo: &impl CategoryRepository,
    id: i32,
) -> Result<CategoryEditView> {
    let all_categories = repo.all().await?;
    let current_name = repo.find(id).await?;
    Ok(CategoryEditView {
        all_categories,
        current_name,
    })
}
