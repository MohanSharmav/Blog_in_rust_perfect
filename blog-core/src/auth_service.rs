//! Registration/authentication use cases.

use crate::error::Result;
use crate::ports::PasswordCipher;
use blog_storage::ports::UserRepository;

pub async fn register(
    users: &impl UserRepository,
    cipher: &impl PasswordCipher,
    username: &str,
    password: &str,
) -> Result<()> {
    let password_hash = cipher.encrypt(password);
    Ok(users.register(username, password_hash).await?)
}

/// Returns `true` when `username`/`password` match a stored account.
pub async fn authenticate(
    users: &impl UserRepository,
    cipher: &impl PasswordCipher,
    username: &str,
    password: &str,
) -> Result<bool> {
    let password_hash = cipher.encrypt(password);
    Ok(users.credentials_match(username, password_hash).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fakes::{FakeCipher, InMemoryUserRepository};

    #[tokio::test]
    async fn registered_user_can_authenticate() {
        let users = InMemoryUserRepository::new();
        let cipher = FakeCipher;

        register(&users, &cipher, "alice", "hunter2").await.unwrap();

        assert!(authenticate(&users, &cipher, "alice", "hunter2")
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn wrong_password_does_not_authenticate() {
        let users = InMemoryUserRepository::new();
        let cipher = FakeCipher;

        register(&users, &cipher, "alice", "hunter2").await.unwrap();

        assert!(!authenticate(&users, &cipher, "alice", "wrong-password")
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn unknown_user_does_not_authenticate() {
        let users = InMemoryUserRepository::new();
        let cipher = FakeCipher;

        assert!(!authenticate(&users, &cipher, "nobody", "anything")
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn password_is_never_stored_in_plaintext() {
        let users = InMemoryUserRepository::new();
        let cipher = FakeCipher;

        register(&users, &cipher, "alice", "hunter2").await.unwrap();

        // credentials_match only succeeds against the *encrypted* form —
        // the plaintext password itself was never passed to the repository.
        assert!(!users
            .credentials_match("alice", "hunter2".to_string())
            .await
            .unwrap());
    }
}
