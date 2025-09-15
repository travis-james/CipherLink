use aes_gcm::{
    Aes256Gcm, Error,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use base64::{Engine, engine::general_purpose};
use sha2::{
    Digest, Sha256,
};

pub struct EncryptData {
    pub hashed_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub cipher_text: String,
}

fn encrypt(plain_text: &str, key: &str) -> Result<EncryptData, aes_gcm::Error> {
    // Make a 32-byte key from the user supplied key.
    let hashed_key = Sha256::digest(key.as_bytes());
    let cipher = Aes256Gcm::new(&hashed_key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, plain_text.as_bytes())?;

    // String can only handle UTF-8 text. ciphertext is raw bytes
    // that can include invalid UTF-8, thus being misrepresented
    // in String format. Since raw bytes are just binary, encode
    // it in base64 that is text safe.
    Ok(EncryptData {
        hashed_key: hashed_key.to_vec(),
        nonce: nonce.to_vec(),
        cipher_text: general_purpose::STANDARD.encode(ciphertext),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_encrypt() {
    //     let url =
    //         "https://docs.rs/aes-gcm/0.10.3/aes_gcm/#in-place-usage-eliminates-alloc-requirement";
    //     let key = "test";
    //     let got = encrypt(url, key).expect("encryption failed");
    //     let plaintext = got.cipher_text.decrypt(got.nonce, ciphertext.as_ref())?;
    // }
}
