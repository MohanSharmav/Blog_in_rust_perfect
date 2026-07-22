//! Shared black-box test support: builds and spawns the real `blog-server`
//! and `blog-cli` binaries and hands back a running server to test against.
//!
//! These are genuine subprocesses talking real HTTP/SQLite, not mocks — the
//! point of this crate is to catch wiring mistakes that only show up when
//! the pieces actually run together (wrong bind address, cookie/CORS
//! settings, route typos, CLI argument mismatches, ...).

use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::{Duration, Instant};

/// A running `blog-server` instance, backed by a scratch SQLite database
/// that's deleted when this (and its tempdir) drop. Killed on drop.
pub struct TestServer {
    child: Child,
    pub base_url: String,
    _db_dir: tempfile::TempDir,
}

impl TestServer {
    /// Builds (if needed) and starts `blog-server` on an ephemeral port,
    /// with a fresh SQLite database, and waits until it's accepting
    /// requests. `cors_for_local_development` is enabled so the session
    /// cookie isn't marked `Secure` — these tests talk plain `http://`.
    pub async fn spawn() -> Self {
        let port = free_port();
        let base_url = format!("http://127.0.0.1:{port}");
        let db_dir = tempfile::tempdir().expect("create scratch db dir");
        let db_path = db_dir.path().join("test.db");

        let child = Command::new(server_binary_path())
            .env("DATABASE_URL", format!("sqlite://{}", db_path.display()))
            .env("MAGIC_KEY", "blog-tests-integration-key")
            .env("BIND_ADDR", format!("127.0.0.1:{port}"))
            .env("RUST_LOG", "error")
            .spawn()
            .expect("failed to start blog-server subprocess");

        let server = Self {
            child,
            base_url,
            _db_dir: db_dir,
        };
        server.wait_until_ready().await;
        server
    }

    async fn wait_until_ready(&self) {
        let deadline = Instant::now() + Duration::from_secs(20);
        let client = reqwest::Client::new();
        let url = format!("{}/posts/page/1", self.base_url);
        loop {
            if let Ok(resp) = client.get(&url).send().await {
                if resp.status().is_success() {
                    return;
                }
            }
            if Instant::now() >= deadline {
                panic!("blog-server did not become ready within 20s");
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind an ephemeral port")
        .local_addr()
        .expect("read local_addr")
        .port()
}

/// Builds (or reuses the cached build of) the `blog-server` binary with the
/// `sqlite` + `cors_for_local_development` features and returns its path.
pub fn server_binary_path() -> PathBuf {
    escargot::CargoBuild::new()
        .manifest_path(workspace_root().join("Cargo.toml"))
        .bin("blog-server")
        .no_default_features()
        .features("sqlite,cors_for_local_development")
        .run()
        .expect("build blog-server for integration tests")
        .path()
        .to_path_buf()
}

/// Builds (or reuses the cached build of) the `blog-cli` binary and returns
/// its path.
pub fn cli_binary_path() -> PathBuf {
    escargot::CargoBuild::new()
        .manifest_path(workspace_root().join("blog-cli").join("Cargo.toml"))
        .bin("blog-cli")
        .run()
        .expect("build blog-cli for integration tests")
        .path()
        .to_path_buf()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("blog-tests has a parent directory")
        .to_path_buf()
}
