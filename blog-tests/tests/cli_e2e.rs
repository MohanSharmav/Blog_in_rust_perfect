//! Black-box test of `blog-cli`: spawns a real `blog-server` and runs the
//! actual `blog-cli` binary as a subprocess against it, asserting on stdout
//! the way a user would read it.

use assert_cmd::Command;
use blog_tests::TestServer;
use predicates::prelude::*;

/// Runs `blog-cli` against `server`, with its session file isolated to
/// `config_dir` so this never touches a real developer's login session.
fn cli(server: &TestServer, config_dir: &std::path::Path) -> Command {
    let mut cmd = Command::new(blog_tests::cli_binary_path());
    cmd.env("BLOG_CLI_CONFIG_DIR", config_dir)
        .arg("--server")
        .arg(&server.base_url);
    cmd
}

/// Parses `"<id>: <name>"` lines as printed by `category list`/`post list`,
/// returning the id for the line whose name matches, plus the total page
/// count parsed from the `-- page P/T (N total) --` trailer.
fn parse_list(list_output: &str, name: &str) -> (Option<i32>, usize) {
    let mut id = None;
    let mut total_pages = 1;
    for line in list_output.lines() {
        if let Some((candidate_id, rest)) = line.split_once(": ") {
            if rest == name {
                id = candidate_id.trim().parse().ok();
            }
        } else if let Some(pages) = line
            .strip_prefix("-- page ")
            .and_then(|rest| rest.split('/').nth(1))
            .and_then(|rest| rest.split_whitespace().next())
        {
            total_pages = pages.parse().unwrap_or(1);
        }
    }
    (id, total_pages)
}

/// Categories/posts are paginated (3 per page) and the seed data already
/// fills page 1, so a newly created item can land on a later page — search
/// every page rather than assuming page 1.
fn find_id(server: &TestServer, config_dir: &std::path::Path, kind: &str, name: &str) -> i32 {
    let mut page = 1u32;
    loop {
        let output = cli(server, config_dir)
            .args([kind, "list", "--page", &page.to_string()])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        let (id, total_pages) = parse_list(&String::from_utf8(output).unwrap(), name);
        if let Some(id) = id {
            return id;
        }
        assert!(
            (page as usize) < total_pages,
            "{kind} {name:?} not found in any page"
        );
        page += 1;
    }
}

#[tokio::test]
async fn cli_auth_and_content_lifecycle() {
    let server = TestServer::spawn().await;
    let config_dir = tempfile::tempdir().expect("scratch config dir");

    // whoami with no session fails.
    cli(&server, config_dir.path())
        .arg("whoami")
        .assert()
        .failure();

    cli(&server, config_dir.path())
        .args(["register", "--username", "alice", "--password", "hunter2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Registered alice"));

    cli(&server, config_dir.path())
        .args(["login", "--username", "alice", "--password", "hunter2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Logged in as alice"));

    // The session persisted to config_dir carries into this new invocation.
    cli(&server, config_dir.path())
        .arg("whoami")
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"));

    // Category lifecycle.
    cli(&server, config_dir.path())
        .args(["category", "create", "--name", "rust-lang"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Category created"));

    let category_id = find_id(&server, config_dir.path(), "category", "rust-lang");

    cli(&server, config_dir.path())
        .args([
            "category",
            "update",
            &category_id.to_string(),
            "--name",
            "systems-programming",
        ])
        .assert()
        .success();

    cli(&server, config_dir.path())
        .args(["category", "get", &category_id.to_string()])
        .assert()
        .success()
        .stdout(predicate::str::contains("systems-programming"));

    // Post lifecycle, attached to that category.
    cli(&server, config_dir.path())
        .args([
            "post",
            "create",
            "--title",
            "Hello CLI",
            "--description",
            "written by blog-tests",
            "--category",
            &category_id.to_string(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Post created"));

    let post_id = find_id(&server, config_dir.path(), "post", "Hello CLI");

    cli(&server, config_dir.path())
        .args([
            "post",
            "update",
            &post_id.to_string(),
            "--title",
            "Hello CLI (edited)",
            "--description",
            "still written by blog-tests",
            "--category",
            &category_id.to_string(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated"));

    cli(&server, config_dir.path())
        .args(["post", "get", &post_id.to_string()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello CLI (edited)"));

    cli(&server, config_dir.path())
        .args(["post", "delete", &post_id.to_string()])
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    cli(&server, config_dir.path())
        .args(["category", "delete", &category_id.to_string()])
        .assert()
        .success();

    cli(&server, config_dir.path())
        .arg("logout")
        .assert()
        .success()
        .stdout(predicate::str::contains("Logged out"));

    // Session is gone: whoami fails again.
    cli(&server, config_dir.path())
        .arg("whoami")
        .assert()
        .failure();
}
