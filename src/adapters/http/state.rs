use crate::adapters::crypto::MagicCryptCipher;
use crate::adapters::postgres::category_repository::PgCategoryRepository;
use crate::adapters::postgres::post_repository::PgPostRepository;
use crate::adapters::postgres::user_repository::PgUserRepository;

/// The composed application state: one concrete adapter per port, wired once
/// at the composition root (`main.rs`) and shared across requests.
#[derive(Clone)]
pub struct AppState {
    pub posts: PgPostRepository,
    pub categories: PgCategoryRepository,
    pub users: PgUserRepository,
    pub cipher: MagicCryptCipher,
}
