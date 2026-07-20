//! Registration/authentication use cases.

use crate::application::ports::{PasswordCipher, UserRepository};
use anyhow::Result;

pub async fn register(
    users: &impl UserRepository,
    cipher: &impl PasswordCipher,
    username: &str,
    password: &str,
) -> Result<()> {
    let password_hash = cipher.encrypt(password);
    users.register(username, password_hash).await
}

/// Returns `true` when `username`/`password` match a stored account.
pub async fn authenticate(
    users: &impl UserRepository,
    cipher: &impl PasswordCipher,
    username: &str,
    password: &str,
) -> Result<bool> {
    let password_hash = cipher.encrypt(password);
    users.credentials_match(username, password_hash).await
}
