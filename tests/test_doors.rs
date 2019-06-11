use std::{thread, time};
extern crate open_taffeta_lib;
extern crate reqwest;
#[macro_use]
extern crate serde_json;

use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::{Client, StatusCode};
use open_taffeta_lib::serializers::doors::*;
use open_taffeta_lib::models::*;
use crate::common::dbstate::DbState;
mod common;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_door_bad_auth() {
    DbState::new();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let payload = json!({
        "name":"door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let mut response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_static("hahaha"))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let resp_str: &str = &response.text().unwrap().to_string();
    assert_eq!(resp_str.contains("Not authorized"), true);
}

#[test]
fn test_door_create() {
    let state = DbState::new();
    let api_base_uri = common::api_base_url();
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_ADMIN);
    let client = Client::new();

    // check for 0 doors
    state.assert_empty_doors();

    let payload = json!({
        "name":"door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let response = client
        .post(api_base_uri.join("/door").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // check new door
    let response = client
        .get(api_base_uri.join("/doors").unwrap())
        .json(&payload)
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_door_delete() {
    let state = DbState::new();
    let api_base_uri = common::api_base_url();
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_ADMIN);
    let client = Client::new();
    // check for 0 doors
    state.assert_empty_doors();

    let payload = json!({
        "name":"door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let door_data = common::create_door(&client, &payload, &token, StatusCode::CREATED)
        .expect("Failed to parse response");

    // check new door
    let res = common::delete_door(&client, door_data.door.id, &token, StatusCode::NO_CONTENT);
    assert!(true, res.is_ok());
}

// TODO: mock this test
// #[test]
fn test_door_buzz() {
    let state = DbState::new();
    let (_, _, token_user) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let (_, _, token_admin) = common::signup_user(&state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let client = Client::new();
    let payload = json!({
        "name": "door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let door_data = common::create_door(&client, &payload, &token_admin, StatusCode::CREATED)
        .expect("Failed to parse response");

    // user cannot ONLY buzz a door
    let max = 5;
    let mut i = 0;
    let sleep_time = time::Duration::from_millis(1500);
    while i < max {
        let res = common::knock_door(&client, door_data.door.id, &token_user, StatusCode::CREATED);
        assert!(true, res.is_ok());
        i += 1;
        thread::sleep(sleep_time);
    }
}

#[test]
fn test_door_inactive_admin_unauthorized() {
    let state = DbState::new();
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", false, ROLE_ADMIN);
    let client = Client::new();
    let payload = json!({
        "name": "door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let res = common::create_door(&client, &payload, &token, StatusCode::UNAUTHORIZED);
    assert!(true, res.is_err());
}

#[test]
fn test_door_inactive_user_cannot_buzz_door() {
    let state = DbState::new();
    let (_, _, token_admin) =
        common::signup_user(&state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let (_, _, token_user) = common::signup_user(&state.conn, "user@domain.com", false, ROLE_USER);
    let client = Client::new();
    let payload = json!({
        "name": "door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let res = common::create_door(&client, &payload, &token_admin, StatusCode::CREATED);
    assert!(true, res.is_ok());
    let door_data = res.unwrap();

    let res = common::knock_door(
        &client,
        door_data.door.id,
        &token_user,
        StatusCode::UNAUTHORIZED,
    );
    assert!(true, res.is_ok());
}

#[test]
fn test_door_user() {
    let state = DbState::new();
    let (_, _, token_user) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let (_, _, token_admin) =
        common::signup_user(&state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let client = Client::new();
    let payload = json!({
        "name": "door123",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let door_data = common::create_door(&client, &payload, &token_admin, StatusCode::CREATED)
        .expect("Failed to parse response");

    // User cannot create a door
    let payload = json!({
        "name": "door456",
        "address": "https://buzzer.somewhere.de",
        "buzzer_url": "http://111.222.111.222"
    });
    let res = common::create_door(&client, &payload, &token_user, StatusCode::UNAUTHORIZED);
    assert!(true, res.is_err());

    // user cannot delete a door
    let res = common::delete_door(&client, door_data.door.id, &token_user, StatusCode::UNAUTHORIZED);
    assert!(true, res.is_err());
}
