use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::crypto::EncryptData;

pub fn encrypt_data_to_item(id: &str, data: &EncryptData) -> HashMap<String, AttributeValue> {
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S(id.to_string()));
    item.insert(
        "hashed_key".to_string(),
        AttributeValue::B(data.hashed_key.clone().into()),
    );
    item.insert(
        "nonce".to_string(),
        AttributeValue::B(data.nonce.clone().into()),
    );
    item.insert(
        "cipher_text".to_string(),
        AttributeValue::B(data.cipher_text.clone().into()),
    );
    item
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
            hashed_key: vec![0x01, 0x02, 0x03],
            nonce: vec![0x04, 0x05, 0x06],
            cipher_text: vec![0x07, 0x08, 0x09],
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
}
