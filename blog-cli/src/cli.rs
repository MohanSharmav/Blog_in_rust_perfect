use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "blog-cli",
    version,
    about = "Command-line client for the blog server"
)]
pub struct Cli {
    /// Base URL of the blog server.
    #[arg(long, global = true, default_value = "http://127.0.0.1:8080")]
    pub server: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Log in and persist the session for later commands.
    Login {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    /// Forget the persisted session.
    Logout,
    /// Show the currently logged-in user.
    Whoami,
    /// Create a new account.
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    /// Manage posts.
    #[command(subcommand)]
    Post(PostCommand),
    /// Manage categories.
    #[command(subcommand)]
    Category(CategoryCommand),
}

#[derive(Subcommand)]
pub enum PostCommand {
    /// List posts.
    List {
        #[arg(long, default_value_t = 1)]
        page: i64,
    },
    /// Show a single post.
    Get { id: i32 },
    /// Create a post (requires login).
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
        /// Category id, omit for no category.
        #[arg(long, default_value_t = 0)]
        category: i32,
    },
    /// Update a post (requires login).
    Update {
        id: i32,
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
        #[arg(long, default_value_t = 0)]
        category: i32,
    },
    /// Delete a post (requires login).
    Delete { id: i32 },
}

#[derive(Subcommand)]
pub enum CategoryCommand {
    /// List categories.
    List {
        #[arg(long, default_value_t = 1)]
        page: i64,
    },
    /// Show a single category.
    Get { id: i32 },
    /// Create a category (requires login).
    Create {
        #[arg(long)]
        name: String,
    },
    /// Rename a category (requires login).
    Update {
        id: i32,
        #[arg(long)]
        name: String,
    },
    /// Delete a category (requires login).
    Delete { id: i32 },
}
