//! Ports owned by this crate. The storage ports (`PostRepository`,
//! `CategoryRepository`, `UserRepository`) live in the `blog-storage` crate
//! alongside their Postgres/SQLite implementations — depend on
//! `blog_storage::ports` for those.

pub trait PasswordCipher {
    fn encrypt(&self, plain: &str) -> String;
}
