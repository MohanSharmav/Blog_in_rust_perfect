use crate::cli::{CategoryCommand, PostCommand};
use anyhow::Result;
use blog_client::{BlogClient, NewPost};

pub async fn run_post(client: &BlogClient, command: PostCommand) -> Result<()> {
    match command {
        PostCommand::List { page } => {
            let listing = client.list_posts(page).await?;
            for post in &listing.items {
                println!("{}: {}", post.id, post.title);
            }
            println!(
                "-- page {}/{} ({} total) --",
                listing.page, listing.total_pages, listing.total_items
            );
        }
        PostCommand::Get { id } => {
            let post = client.get_post(id).await?;
            println!("id: {}", post.id);
            println!("title: {}", post.title);
            println!("description: {}", post.description);
        }
        PostCommand::Create {
            title,
            description,
            category,
        } => {
            client
                .create_post(&NewPost {
                    title,
                    description,
                    category_id: category,
                })
                .await?;
            println!("Post created.");
        }
        PostCommand::Update {
            id,
            title,
            description,
            category,
        } => {
            client
                .update_post(
                    id,
                    &NewPost {
                        title,
                        description,
                        category_id: category,
                    },
                )
                .await?;
            println!("Post {id} updated.");
        }
        PostCommand::Delete { id } => {
            client.delete_post(id).await?;
            println!("Post {id} deleted.");
        }
    }
    Ok(())
}

pub async fn run_category(client: &BlogClient, command: CategoryCommand) -> Result<()> {
    match command {
        CategoryCommand::List { page } => {
            let listing = client.list_categories(page).await?;
            for category in &listing.items {
                println!("{}: {}", category.id, category.name);
            }
            println!(
                "-- page {}/{} ({} total) --",
                listing.page, listing.total_pages, listing.total_items
            );
        }
        CategoryCommand::Get { id } => {
            let category = client.get_category(id).await?;
            println!("id: {}", category.id);
            println!("name: {}", category.name);
        }
        CategoryCommand::Create { name } => {
            client.create_category(&name).await?;
            println!("Category created.");
        }
        CategoryCommand::Update { id, name } => {
            client.update_category(id, &name).await?;
            println!("Category {id} updated.");
        }
        CategoryCommand::Delete { id } => {
            client.delete_category(id).await?;
            println!("Category {id} deleted.");
        }
    }
    Ok(())
}
