extern crate rand;
extern crate crypto;

use common::rand::distributions::Alphanumeric;
use common::rand::{thread_rng, OsRng, Rng};
use common::crypto::pbkdf2::pbkdf2_simple;

use std::env;

use reqwest::Url;

pub mod dbstate;

pub fn api_base_url() -> Url {
    let server_base_url = match env::var("TEST_SERVER") {
        Err(_) => "http://localhost:8080".to_string(),
        Ok(uri) => uri,
    };
    Url::parse(&server_base_url).unwrap()
}

// fn auth_token_headers() -> String {
//     // let (user, password) = common::valid_user();
//     // let api_base_uri = common::api_base_url();

//     let test_client = Client::new();
//     let auth_url = api_base_uri.join("/auth").unwrap();
//     let credentials = json!({"username": user.username, "password": password});

//     let mut auth_response = test_client
//         .post(auth_url)
//         .json(&credentials)
//         .send()
//         .unwrap();

//     let token = auth_response.json::<Value>().unwrap()["access_token"]
//         .as_str()
//         .unwrap()
//         .to_string();
//     format!("JWT {}", token)
// }

/// Generate an auth token and save it to the `current_auth_token` column.
fn generate_auth_token() -> String {
    let mut rand_gen = OsRng::new().expect("Couldn't make OsRng!");
    let new_auth_token = rand_gen
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect::<String>();
    new_auth_token
}

// TODO: return a user instance + auth token
// TODO: see rust-web-boilerplate/src/models/user.rs
pub fn signup_user(username: String, password: String) -> String {
    let user = format!("{}::{}::{}", username, password, generate_auth_token());
    user
}

pub fn generate_password() -> String {
    let password: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    pbkdf2_simple(&password, 5000).unwrap()

}
