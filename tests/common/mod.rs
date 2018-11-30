extern crate rand;
extern crate crypto;

use common::rand::distributions::Alphanumeric;
use common::rand::{thread_rng, Rng};
use common::crypto::pbkdf2::pbkdf2_simple;

use std::env;

use reqwest::{Url, Client, StatusCode};

extern crate rocket_contrib;
use common::rocket_contrib::{Value};

pub mod dbstate;

pub fn api_base_url() -> Url {
    let server_base_url = match env::var("TEST_SERVER") {
        Err(_) => "http://localhost:8080".to_string(),
        Ok(uri) => uri,
    };
    Url::parse(&server_base_url).unwrap()
}

pub fn signup_user(username: &str, email: &str) -> (Value, String) {
    let client = Client::new();
    let api_base_uri = api_base_url();
    let user_data = json!({
        "username": username,
        "password": generate_password(),
        "email": email
    });
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_str : &str = &response.text().unwrap().to_string();
    let resp_data : Value = serde_json::from_str(resp_str).unwrap();
    let resp_user_data = resp_data["user"].clone();
    let token = String::from(resp_data["user"]["token"].as_str().unwrap());
    (resp_user_data, token)
}

pub fn generate_password() -> String {
    let password: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    pbkdf2_simple(&password, 5000).unwrap()
}
