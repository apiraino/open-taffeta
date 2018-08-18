use rand::distributions::Alphanumeric;
use rand::{OsRng, Rng};

use reqwest::Url;

pub fn api_base_url() -> Url {
    let server_base_url = dotenv!("TEST_SERVER");
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

pub fn setup() {}

pub fn teardown() {}
