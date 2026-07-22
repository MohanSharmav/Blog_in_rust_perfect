//! Category-related use cases.

use crate::error::Result;
use crate::pagination::{resolve_page, Listing, DEFAULT_PAGE_SIZE};
use blog_storage::domain::category::Category;
use blog_storage::ports::CategoryRepository;

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
    Ok(repo.create(name).await?)
}

pub async fn update_category(repo: &impl CategoryRepository, id: i32, name: &str) -> Result<()> {
    Ok(repo.update(id, name).await?)
}

pub async fn delete_category(repo: &impl CategoryRepository, id: i32) -> Result<()> {
    Ok(repo.delete(id).await?)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fakes::InMemoryCategoryRepository;

    #[tokio::test]
    async fn list_categories_resolves_none_past_the_last_page() {
        let repo = InMemoryCategoryRepository::new();
        for i in 0..DEFAULT_PAGE_SIZE {
            repo.create(&format!("category {i}")).await.unwrap();
        }

        assert!(list_categories(&repo, 1).await.unwrap().is_some());
        assert!(list_categories(&repo, 2).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn list_categories_on_an_empty_repo_resolves_none() {
        let repo = InMemoryCategoryRepository::new();
        // Unlike `list_posts_for_category`, plain `list_categories` doesn't
        // clamp to a minimum of one page — an empty listing is out of range.
        assert!(list_categories(&repo, 1).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn create_update_delete_roundtrip() {
        let repo = InMemoryCategoryRepository::new();
        create_category(&repo, "rust").await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 1);

        update_category(&repo, 1, "rust-lang").await.unwrap();
        assert_eq!(repo.find(1).await.unwrap()[0].name, "rust-lang");

        delete_category(&repo, 1).await.unwrap();
        assert!(repo.find(1).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn category_for_edit_reports_current_name_and_every_category() {
        let repo = InMemoryCategoryRepository::new();
        create_category(&repo, "sports").await.unwrap();
        create_category(&repo, "travel").await.unwrap();

        let view = category_for_edit(&repo, 1).await.unwrap();
        assert_eq!(view.current_name[0].name, "sports");
        assert_eq!(view.all_categories.len(), 2);
        assert!(view.all_categories.iter().any(|c| c.name == "travel"));
    }
}
