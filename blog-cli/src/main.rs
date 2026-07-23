mod cli;
mod commands;
mod session;

use anyhow::Result;
use blog_client::BlogClient;
use clap::Parser;
use cli::{Cli, Command};
use session::Session;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut session = Session::load(&cli.server)?;

    let client = match session.cookie() {
        Some(cookie) => BlogClient::with_session(cli.server.clone(), cookie),
        None => BlogClient::new(cli.server.clone()),
    };

    match cli.command {
        Command::Login { username, password } => {
            client.login(&username, &password).await?;
            session.set_cookie(client.session_cookie());
            session.save()?;
            println!("Logged in as {username}.");
        }
        Command::Logout => {
            client.logout().await?;
            session.set_cookie(None);
            session.save()?;
            println!("Logged out.");
        }
        Command::Whoami => {
            let username = client.me().await?;
            println!("{username}");
        }
        Command::Register { username, password } => {
            client.register(&username, &password).await?;
            println!("Registered {username}. Run `blog-cli login` to authenticate.");
        }
        Command::Post(command) => commands::run_post(&client, command).await?,
        Command::Category(command) => commands::run_category(&client, command).await?,
    }

    Ok(())
}
