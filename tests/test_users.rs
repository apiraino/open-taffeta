extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use]
extern crate serde_json;
use serde_json::Value;

use reqwest::header::HeaderValue;
use reqwest::{Client, StatusCode};

mod common;

use common::dbstate::DbState;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_db() {
    let state = DbState::new();
    state.create_user("josh@domain.com");
}

#[test]
fn test_create_user() {
    let state = DbState::new();
    state.clean_users();
    let api_base_uri = common::api_base_url();
    let client = Client::new();

    // check for 0 users
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    assert_eq!(resp_data["users"].as_array().unwrap().len(), 0);

    // create a user
    let (test_user, _) = state.create_user("josh@domain.com");

    // check if new user exists
    let url = &format!("/user/{}", test_user.id);
    let mut response = client
        .get(api_base_uri.join(url).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    assert_eq!(resp_data["user"].as_array().unwrap().len(), 1);
    assert_eq!(resp_data["user"][0]["email"], "josh@domain.com");
}

#[test]
fn test_list_users() {
    DbState::new();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    assert_eq!(resp_data["users"].as_array().unwrap().len(), 0);
}

#[test]
fn test_signup_ok() {
    DbState::new();
    let api_base_uri = common::api_base_url();
    let user_data = json!({
        "username": "john",
        "password": "1234567",
        "email": "hey@email.com"
    });
    let client = Client::new();
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    let resp_user_data: Value = resp_data["user"].clone();
    assert_eq!(response.status(), StatusCode::CREATED);
    let user_id = resp_user_data["id"].as_i64().unwrap() as i32;

    let url = &format!("/user/{}", user_id);
    response = client
        .get(api_base_uri.join(url).unwrap())
        .send()
        .expect("thought it worked...");
    assert_eq!(response.status(), StatusCode::OK);
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    assert_eq!(resp_data["user"][0]["id"], user_id);

    // repeat same payload, expect a 400
    response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let resp_str: &str = &response.text().unwrap().to_string();
    assert_eq!(resp_str.contains("record already exists"), true);
}
