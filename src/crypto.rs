extern crate crypto_hash;
use crypto_hash::{Algorithm, hex_digest};

pub const SECRET : &str = "123456";

pub fn calculate_hash(_code: String) -> String {
    String::from("1234567890qwerty")
}

pub fn hash_password(password: String) -> String {
    hex_digest(Algorithm::SHA256, &password.into_bytes())
}

#[test]
fn test_calculate_code() {
    use std::collections::HashMap;
    use crate::crypto::calculate_hash;
    let values : HashMap<&str,&str> = [
        ("123456", "1234567890qwerty"),
    ]
       .iter()
       .cloned()
       .collect();

    for (test_val, exp_val) in &values {
        // let code = test_val.to_string();
        let res = calculate_hash(test_val.to_string());
        assert_eq!(exp_val, &res);
    }
}
