extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use]
extern crate serde_json;

use reqwest::header::Authorization;
use reqwest::{Client, StatusCode};

mod common;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_list_users() {
    // TODO
    let client = common::setup();
    common::teardown();

    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let response = client
        .get(api_base_uri.join("/users").unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
}

#[test]
fn test_signup_ok() {
    extern crate serde_json;
    use serde_json::Value;

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
    println!("STR {:?}", resp_str);
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    let user_data: Value = resp_data["user"].clone();
    println!("JSON {:?}", user_data);
    let resp_data2: open_taffeta_lib::models::User =
        serde_json::from_value(user_data.clone()).unwrap();
    println!("USER {:?}", resp_data2);
    assert_eq!(response.status(), StatusCode::Created);

    let url = &format!("/user/{}", resp_data["user"]["id"]).to_string();
    response = client
        .get(api_base_uri.join(url).unwrap())
        .send()
        .expect("thought it worked...");
    assert_eq!(response.status(), StatusCode::Ok);
    let resp_str: &str = &response.text().unwrap().to_string();
    let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    println!("JSON2 {:?}", resp_data["user"][0]["id"]);
    assert_eq!(resp_data["user"][0]["id"], user_data["id"]);

    response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::BadRequest);
    // let resp_data: Value = serde_json::from_str(resp_str).unwrap();
    assert_eq!(resp_str.contains("record already exists"), true)
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
