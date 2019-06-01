extern crate rand;
extern crate crypto_hash;
extern crate open_taffeta_lib;

use std::env;

use reqwest::{Url, Client, StatusCode};
use reqwest::header::{AUTHORIZATION, HeaderValue};

// need "Value" because serde serializers
// exported from Rocket 0.4 get compiler confused (?)
// use rocket_contrib::json::JsonValue;
use serde_json::Value;
// use serde_derive::{Deserialize};
// use open_taffeta_lib::models::{UserAuth, Door, User};

use open_taffeta_lib::serializers::users::{ResponseUserDetail, ResponseLoginSignup, ResponseError};
use diesel::sqlite::SqliteConnection;

pub mod dbstate;

pub fn api_base_url() -> Url {
    let server_base_url = match env::var("TEST_SERVER") {
        Err(_) => "http://localhost:8080".to_string(),
        Ok(uri) => uri,
    };
    Url::parse(&server_base_url).unwrap()
}

type Password = String;
type Token = String;

pub fn signup_user(conn: &SqliteConnection, email: &str, is_active: bool) ->
    (ResponseUserDetail, Password, Token)
{
    let client = Client::new();
    let api_base_uri = api_base_url();
    let password = open_taffeta_lib::config::generate_password();
    let user_data = json!({
        "password": password,
        "email": email
    });
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_data : ResponseLoginSignup = response.json()
        .expect("Error opening signup response");
    let token = resp_data.auth.token;

    // activate user
    if is_active {
        let user = open_taffeta_lib::models::User {
            id: resp_data.auth.user_id,
            password: password,
            email: email.to_owned(),
            is_active: true
        };
        open_taffeta_lib::db::update_user(&conn, user);
    }

    // get back that user (Sqlite has no RETURNING support)
    let q = format!("/users/{}", resp_data.auth.user_id);
    let mut response = client
        .get(api_base_uri.join(&q).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .expect(&format!("Failed to get user with id={}", resp_data.auth.user_id));
    // eprintln!("Got response: {}", response.status());
    // assert_eq!(response.status(), StatusCode::OK);
    // TODO: here improve the return value
    if response.status() != StatusCode::OK {
        let err_data : ResponseError = response
            .json()
            .expect("Error opening error response");
        eprintln!("Error {} in getting user with id={}: {:?}",
                  response.status(),
                  resp_data.auth.user_id,
                  err_data.detail);
    }
    let resp_data : ResponseUserDetail = response.json()
        .expect("Error opening user detail response");
    (resp_data, user_data.get("password").unwrap().to_string(), token)
}

// enum OkResponse {
//     ResponseUserDetail,
//     ResponseError
// }

pub fn get_user_detail(client: &Client, user_id: i32, auth_token: String, expected_status_code: StatusCode) -> Option<ResponseUserDetail> {
    let api_base_uri = api_base_url();
    let mut response = client
        .get(api_base_uri.join(&format!("/users/{}", user_id)).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(auth_token.as_str()).unwrap())
        .send()
        .expect("Failed request: user detail");
    if response.status() != expected_status_code {
        if response.status() != StatusCode::OK {
            let r : ResponseError = response.json()
                .expect("Error opening user detail response");
            let err_msg = format!(
                "Error in get user detail: expected {}, got {}: {:?}",
                expected_status_code, response.status(),
                r.detail
            );
            panic!(err_msg);
        }
    }
    if response.status() == StatusCode::OK {
        let r : ResponseUserDetail = response.json()
            .expect("Error opening user detail response");
        return Some(r);
    }
    None
}

pub fn user_login(client: &Client, login_data: &Value, expected_status_code: StatusCode) -> Option<ResponseLoginSignup> {
    let api_base_uri = api_base_url();
    let mut response = client
        .post(api_base_uri.join("/login").unwrap())
        .json(&login_data)
        .send()
        .expect("Login failed");
    assert_eq!(response.status(), expected_status_code);
    if response.status() == StatusCode::OK {
        let resp_data: ResponseLoginSignup = response.json().expect("Failed to unwrap the login response");
        return Some(resp_data);
    }
    None
}
