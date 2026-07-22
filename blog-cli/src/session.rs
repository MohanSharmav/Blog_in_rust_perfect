//! Persists the session cookie to a local file between separate CLI
//! invocations, keyed by server URL so `--server` can point at different
//! environments without clobbering each other's sessions.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Session {
    server: String,
    sessions: HashMap<String, String>,
}

impl Session {
    pub fn load(server: &str) -> Result<Self> {
        let path = session_file()?;
        let sessions = match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
        Ok(Self {
            server: server.to_string(),
            sessions,
        })
    }

    pub fn cookie(&self) -> Option<&str> {
        self.sessions.get(&self.server).map(String::as_str)
    }

    pub fn set_cookie(&mut self, cookie: Option<String>) {
        match cookie {
            Some(cookie) => {
                self.sessions.insert(self.server.clone(), cookie);
            }
            None => {
                self.sessions.remove(&self.server);
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = session_file()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        let contents = serde_json::to_string_pretty(&self.sessions)?;
        fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))
    }
}

fn session_file() -> Result<PathBuf> {
    // Overridable so tests can point sessions at a scratch directory instead
    // of clobbering whatever real session is logged in on the host.
    if let Ok(dir) = std::env::var("BLOG_CLI_CONFIG_DIR") {
        return Ok(PathBuf::from(dir).join("session.json"));
    }
    let dirs = ProjectDirs::from("dev", "blog", "blog-cli")
        .context("could not determine a config directory for this platform")?;
    Ok(dirs.config_dir().join("session.json"))
}
