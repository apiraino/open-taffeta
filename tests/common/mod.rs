extern crate rand;
extern crate crypto;
extern crate open_taffeta_lib;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use crypto::pbkdf2::pbkdf2_simple;
use std::env;

use reqwest::{Url, Client, StatusCode};

// need "Value" because serde serializers
// exported from Rocket 0.4 get compiler confused (?)
// use rocket_contrib::json::JsonValue;
use serde_json::Value;
use serde_derive::{Deserialize};
use open_taffeta_lib::models::{UserAuth, Door};

pub mod dbstate;

#[derive(Deserialize, Debug)]
pub struct ResponseSignup {
    pub user: UserAuth
}

#[derive(Deserialize, Debug)]
pub struct ResponseListUser {
    pub users: Vec<User>
}

#[derive(Deserialize, Debug)]
pub struct ResponseUserDetail {
    pub user: User
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub active: bool,
    pub password: String,
    pub email: String
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub detail: String
}

#[derive(Deserialize, Debug)]
pub struct ResponseDoorCreated {
    pub door: Door
}

pub fn api_base_url() -> Url {
    let server_base_url = match env::var("TEST_SERVER") {
        Err(_) => "http://localhost:8080".to_string(),
        Ok(uri) => uri,
    };
    Url::parse(&server_base_url).unwrap()
}

pub fn signup_user(email: &str) -> (Value, String) {
    let client = Client::new();
    let api_base_uri = api_base_url();
    let user_data = json!({
        "password": generate_password(),
        "email": email
    });
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_data : ResponseSignup = response.json().unwrap();
    let token = resp_data.user.token;
    let user_data = json!({
        "id": resp_data.user.id,
        "email": resp_data.user.email
    });
    (user_data, token)
}

pub fn generate_password() -> String {
    let password: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    pbkdf2_simple(&password, 5000).unwrap()
}
