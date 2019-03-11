extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use] extern crate serde_json;

use reqwest::header::{AUTHORIZATION, HeaderValue};
use reqwest::{Client, StatusCode};

mod common;

use crate::common::dbstate::DbState;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_db() {
    let state = DbState::new();
    state.create_user("josh@domain.com", true);
}

#[test]
fn test_create_user() {
    let state = DbState::new();
    state.clean_tables();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    // check for 0 users
    state.assert_empty_users();
    // signup a user
    let (_, token) = common::signup_user("josh@domain.com");
    // check for 1 users
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data : common::ResponseListUser = response.json().unwrap();
    assert_eq!(resp_data.users.len(), 1);
    assert_eq!(resp_data.users[0].email, "josh@domain.com");
}

#[test]
fn test_list_users() {
    let state = DbState::new();
    state.clean_tables();
    state.create_user("inactive@domain.com", false);
    let (_, token) = common::signup_user("josh@domain.com");
    let api_base_uri = common::api_base_url();
    let client = Client::new();

    // query all users
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: common::ResponseListUser = response.json().unwrap();
    assert_eq!(resp_data.users.len(), 2);

    // query only active users
    let mut response = client
        .get(api_base_uri.join("/users?is_active=true").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: common::ResponseListUser = response.json().unwrap();
    assert_eq!(resp_data.users.len(), 1);
}

#[test]
fn test_signup() {
    DbState::new();
    let api_base_uri = common::api_base_url();
    let user_data = json!({
        "password": "1234567",
        "email": "hey@email.com"
    });
    let client = Client::new();
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_data: common::ResponseSignup = response.json().unwrap();
    let user_id = resp_data.user.id;
    let token = resp_data.user.token;

    let url = &format!("/user/{}", user_id);
    response = client
        .get(api_base_uri.join(url).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .expect("thought it worked...");
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: common::ResponseUserDetail = response.json().unwrap();
    assert_eq!(resp_data.user.id, user_id);

    // repeat same payload, expect a 400
    response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let resp_data: common::ResponseError = response.json().unwrap();
    assert_eq!(resp_data.detail.contains("record already exists"), true);
}
