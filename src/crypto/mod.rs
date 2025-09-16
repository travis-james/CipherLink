use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};

pub struct EncryptData {
    pub hashed_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub cipher_text: Vec<u8>,
}

use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EncryptDataForDynamo {
    pub hashed_key: String,
    pub nonce: String,
    pub cipher_text: String,
}

impl From<EncryptData> for EncryptDataForDynamo {
    fn from(data: EncryptData) -> Self {
        EncryptDataForDynamo {
            hashed_key: general_purpose::STANDARD.encode(data.hashed_key),
            nonce: general_purpose::STANDARD.encode(data.nonce),
            cipher_text: general_purpose::STANDARD.encode(data.cipher_text),
        }
    }
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
        // cipher_text: general_purpose::STANDARD.encode(ciphertext),
        cipher_text: ciphertext,
    })
}

fn decrypt(data: &EncryptData) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&data.hashed_key));
    let plaintext = cipher.decrypt(
        GenericArray::from_slice(&data.nonce),
        data.cipher_text.as_ref(),
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
            let got_decryption = decrypt(&got_encryption).expect("decryption failed");

            assert_eq!(plaintext, String::from_utf8(got_decryption).unwrap());

            println!("key: {:?}", got_encryption.hashed_key);
            println!("ciph: {:?}", got_encryption.cipher_text);
            println!("nonce: {:?}", got_encryption.nonce);
        }
    }

    #[test]
    fn deving() {
        let inputs = vec![("foo", "bar"), ("int", "dex")];
        for (plaintext, key) in inputs {
            let encrypted = encrypt(plaintext, key).expect("encryption failed");
            let dynamo_data: EncryptDataForDynamo = encrypted.into();

            let json = serde_json::to_string_pretty(&dynamo_data).unwrap();
            println!("{}", json);
        }
    }
}
