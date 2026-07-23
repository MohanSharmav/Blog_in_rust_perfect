use blog_core::ports::PasswordCipher;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

/// `magic-crypt`-backed adapter implementing the [`PasswordCipher`] port.
#[derive(Clone)]
pub struct MagicCryptCipher {
    key: MagicCrypt256,
}

impl MagicCryptCipher {
    pub fn new(secret: &str) -> Self {
        Self {
            key: new_magic_crypt!(secret, 256),
        }
    }
}

impl PasswordCipher for MagicCryptCipher {
    fn encrypt(&self, plain: &str) -> String {
        self.key.encrypt_str_to_base64(plain)
    }
}
