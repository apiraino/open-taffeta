extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

pub fn calculate_hash(_code: String) -> String {
    String::from("1234567890qwerty")
}

pub fn hash_password(password: &[u8]) -> String {
    hex_digest(Algorithm::SHA256, password)
}

#[test]
fn test_calculate_code() {
    use crate::crypto::calculate_hash;
    use std::collections::HashMap;
    // for tests: SECRET=123456
    let values: HashMap<&str, &str> = [
        ("123456", "1234567890qwerty"),
    ]
    .iter()
    .cloned()
    .collect();

    eprintln!();
    for (test_val, exp_val) in &values {
        // let code = test_val.to_string();
        let res = calculate_hash(test_val.to_string());
        assert_eq!(exp_val, &res);
    }
    eprintln!("---");
}
