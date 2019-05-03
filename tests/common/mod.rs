extern crate rand;
extern crate crypto_hash;
extern crate open_taffeta_lib;

use std::env;

use reqwest::{Url, Client, StatusCode};
use reqwest::header::{AUTHORIZATION, HeaderValue};

// need "Value" because serde serializers
// exported from Rocket 0.4 get compiler confused (?)
// use rocket_contrib::json::JsonValue;
// use serde_json::Value;
// use serde_derive::{Deserialize};
// use open_taffeta_lib::models::{UserAuth, Door, User};

use open_taffeta_lib::serializers::users::{ResponseUserDetail, ResponseLoginSignup};
use diesel::sqlite::SqliteConnection;

pub mod dbstate;

pub fn api_base_url() -> Url {
    let server_base_url = match env::var("TEST_SERVER") {
        Err(_) => "http://localhost:8080".to_string(),
        Ok(uri) => uri,
    };
    Url::parse(&server_base_url).unwrap()
}

pub fn signup_user(conn: &SqliteConnection, email: &str, is_active: bool) -> (ResponseUserDetail, String, String) {
    let client = Client::new();
    let api_base_uri = api_base_url();
    let user_data = json!({
        "password": open_taffeta_lib::config::generate_password(),
        "email": email
    });
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_data : ResponseLoginSignup = response.json().expect("Error opening signup response");
    let token = resp_data.user.token;

    if is_active {
        let user = open_taffeta_lib::models::User {
            id: resp_data.user.user_id,
            password: user_data.get("password").unwrap().to_string(),
            email: resp_data.user.email.clone(),
            active: true
        };
        open_taffeta_lib::db::update_user(&conn, user);
    }

    // query that user
    let q = format!("/users/{}", resp_data.user.user_id);
    let mut response = client
        .get(api_base_uri.join(&q).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data : ResponseUserDetail = response.json().expect("Error opening user detail response");

    (resp_data, user_data.get("password").unwrap().to_string(), token)
}
