use aes_gcm::{
    Aes256Gcm, Error,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use base64::{Engine, engine::general_purpose};
use sha2::{Digest, Sha256};

pub struct CipherLink {
    pub nonce: [u8; 12],
    pub 
}

fn encrypt(plain_text: &str, key: &str) -> Result<String, Error> {
    // Make a 32-byte key from the user supplied key.
    let hashed_key = Sha256::digest(key.as_bytes());
    let cipher = Aes256Gcm::new(&hashed_key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, plain_text.as_bytes())?;

    // String can only handle UTF-8 text. ciphertext is raw bytes
    // that can include invalid UTF-8, thus being misrepresented
    // in String format. Since raw bytes are just binary, encode
    // it in base64 that is text safe.
    return Ok(general_purpose::STANDARD.encode(ciphertext));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let url = "https://docs.rs/aes-gcm/0.10.3/aes_gcm/#in-place-usage-eliminates-alloc-requirement";
        let key = "test";

    }
}