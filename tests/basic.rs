extern crate open_taffeta_lib;

#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate serde_json;

use reqwest::header::Authorization;
use reqwest::{Client, Response, StatusCode};

mod common;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_list_users() {
    // TODO
    // common::setup();
    // common::teardown();

    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let response = client
        .get(api_base_uri.join("/users").unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
}

#[test]
fn test_signup() {
    let api_base_uri = common::api_base_url();
    let credentials = json!({"username": "antonio", "password": "1234567"});
    let client = Client::new();
    let response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&credentials)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::Created);
}

#[test]
fn test_bad_auth() {
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let payload = json!({"name":"door123"});
    let response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(Authorization("hahaha".to_string()))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::Unauthorized);
}
