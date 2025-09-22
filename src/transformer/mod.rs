use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::crypto::EncryptData;

pub fn encrypt_data_to_item(id: &str, data: &EncryptData) -> HashMap<String, AttributeValue> {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S(id.to_string()));
    item.insert(
        "nonce".to_string(),
        AttributeValue::B(data.nonce.clone().into()),
    );
    item.insert(
        "cipher_text".to_string(),
        AttributeValue::B(data.encrypted_text.clone().into()),
    );
    item
}

pub fn item_to_encryt_data(item: &HashMap<String, AttributeValue>) -> Result<EncryptData, String> {
    let nonce = match item.get("nonce") {
        Some(AttributeValue::B(bytes)) => bytes.as_ref().to_vec(),
        _ => return Err("Missing or invalid 'nonce'".into()),
    };
    let cipher_text = match item.get("cipher_text") {
        Some(AttributeValue::B(bytes)) => bytes.as_ref().to_vec(),
        _ => return Err("Missing or invalid 'cipher_text'".into()),
    };
    Ok(EncryptData {
        nonce,
        encrypted_text: cipher_text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_data_to_item() {
        // Not really a meaningful test, just wanted to have a test module
        // for when I do add more tests.
        let id = "5";
        let data = &EncryptData {
            nonce: vec![0x04, 0x05, 0x06],
            encrypted_text: vec![0x07, 0x08, 0x09],
        };
        let got = encrypt_data_to_item(id, data);
        let expected_len = 4;
        assert_eq!(
            expected_len,
            got.len(),
            "expected length: {}, got: {}",
            expected_len,
            got.len()
        );
    }

    #[test]
    fn test_item_to_encrypt_data() {
        let id = "5";
        let data = &EncryptData {
            nonce: vec![0x04, 0x05, 0x06],
            encrypted_text: vec![0x07, 0x08, 0x09],
        };
        let item = encrypt_data_to_item(id, data);
        let got = item_to_encryt_data(&item).expect("failed to transform");
        assert_eq!(
            data.nonce, got.nonce,
            "expected nonce: {:?}, got: {:?}",
            data.nonce, got.nonce,
        );
        assert_eq!(
            data.encrypted_text, got.encrypted_text,
            "expected cipher_text: {:?}, got: {:?}",
            data.encrypted_text, got.encrypted_text,
        )
    }
}
