use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};

pub struct EncryptData {
    pub nonce: Vec<u8>,
    pub encrypted_text: Vec<u8>,
}

/// Encrypts the user provided plain_text with the given key.
/// Returns the encrypted text and nonce used during encryption as a struct.
///
/// # Errors
/// Returns an error if encryption fails.
///
/// # Safety
/// This function does not panic under normal conditions.
pub fn encrypt(plain_text: &str, key: &str) -> Result<EncryptData, aes_gcm::Error> {
    // Make a 32-byte key from the user supplied key, otherwise AES256GCM panics.
    let derived_key = Sha256::digest(key.as_bytes());
    let cipher = Aes256Gcm::new(&derived_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plain_text.as_bytes())?;

    Ok(EncryptData {
        nonce: nonce.to_vec(),
        encrypted_text: ciphertext,
    })
}

/// Decrypts the user provided encrypted text with the user provided key
/// and nonce. Returns the plain text of the provided encrypted text.
///
/// # Errors
/// Returns an error if decryption fails.
///
/// # Safety
/// This function does not panic under normal conditions.
pub fn decrypt(data: &EncryptData, key: &str) -> Result<Vec<u8>, aes_gcm::Error> {
    // derive the key again.
    let derived_key = Sha256::digest(key.as_bytes());
    let cipher = Aes256Gcm::new(&derived_key.into());

    let plaintext = cipher.decrypt(
        GenericArray::from_slice(&data.nonce),
        data.encrypted_text.as_ref(),
    )?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let tests = vec![
            (
                "https://docs.rs/aes-gcm/0.10.3/aes_gcm/#in-place-usage-eliminates-alloc-requirement",
                "test",
            ),
            ("https://docs.rs/aes-gcm/latest/aes_gcm/#usage", "foo"),
            ("abc", "bar"),
        ];
        for (plaintext, key) in tests {
            let got_encryption = encrypt(plaintext, key).expect("encryption failed");
            let got_decryption = decrypt(&got_encryption, key).expect("decryption failed");

            assert_eq!(plaintext, String::from_utf8(got_decryption).unwrap());

            println!("ciph: {:?}", got_encryption.encrypted_text);
            println!("nonce: {:?}", got_encryption.nonce);
        }
    }
}
