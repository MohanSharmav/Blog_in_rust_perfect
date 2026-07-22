use serde::Deserialize;

/// A username/password pair, as submitted by a login or registration form.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
