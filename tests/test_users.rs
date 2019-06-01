extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use] extern crate serde_json;

use reqwest::header::{AUTHORIZATION, HeaderValue};
use reqwest::{Client, StatusCode};

mod common;

use crate::common::dbstate::DbState;
use open_taffeta_lib::serializers::users::{ResponseUserDetail, ResponseListUser, ResponseLoginSignup, ResponseError};

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_db() {
    let state = DbState::new();
    state.create_user("josh@domain.com", true);
}

#[test]
fn test_user_signup() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
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
    let resp_data: ResponseLoginSignup = response.json().expect("Error unwrapping signup response");
    let user_id = resp_data.auth.user_id;
    let token = resp_data.auth.token;
    assert_eq!(resp_data.user.id, user_id);
    assert_eq!(resp_data.user.is_active, false);
    assert_eq!(resp_data.user.role, "user");

    // should count 1 user
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data : ResponseListUser = response.json().unwrap();
    assert_eq!(resp_data.users.len(), 1);
    assert_eq!(resp_data.users[0].email, "hey@email.com");
    assert_eq!(resp_data.users[0].role, "user");
}

#[test]
fn test_user_signup_and_activate() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    // signup a user
    let (resp_data, _, _) = common::signup_user(&state.conn, "josh@domain.com", false);
    assert_eq!(resp_data.user.email, "josh@domain.com");
    assert_eq!(resp_data.user.is_active, false);

    let (resp_data, _, _) = common::signup_user(&state.conn, "josh1@domain.com", true);
    assert_eq!(resp_data.user.email, "josh1@domain.com");
    assert_eq!(resp_data.user.is_active, true);
}

#[test]
fn test_user_already_signed_up() {
    let state = DbState::new();
    state.clean_tables();
    // check for 0 users
    state.assert_empty_users();
    // signup a user
    let (resp_data, _, _) = common::signup_user(&state.conn, "josh@domain.com", false);
    assert_eq!(resp_data.user.email, "josh@domain.com");
    assert_eq!(resp_data.user.is_active, false);

    let (resp_data, _, _) = common::signup_user(&state.conn, "josh1@domain.com", true);
    assert_eq!(resp_data.user.email, "josh1@domain.com");
    assert_eq!(resp_data.user.is_active, true);

    // repeat same payload, expect a 400
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let user_data = json!({
        "password": "1234567",
        "email": "josh1@domain.com"
    });
    let mut response = client
        .post(api_base_uri.join("/signup").unwrap())
        .json(&user_data)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let resp_data: ResponseError = response.json().expect("Error reading error response");
    assert_eq!(resp_data.detail.contains("record already exists"), true);
}

#[test]
fn test_user_detail() {
    let state = DbState::new();
    state.clean_tables();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    // check for 0 users
    state.assert_empty_users();
    // signup a user
    let (user_data, _, token) = common::signup_user(&state.conn, "josh@domain.com", true);

    // get user detail
    let q = format!("/users/{}", user_data.user.id);
    let mut response = client
        .get(api_base_uri.join(&q).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data : ResponseUserDetail = response.json().unwrap();
    assert_eq!(resp_data.user.email, "josh@domain.com");
}

#[test]
fn test_user_detail_not_allowed() {
    assert!(true, "TODO: user2 should not be allowed to access user1 details");
}

#[test]
fn test_user_no_allowed_to_admin_interface() {
    // hint: use Rocket route ordering
    assert!(true, "TODO: user is not allowed to any admin interface (such as /users)");
}

#[test]
fn test_admin_update_allowed() {
    assert!(true, "TODO: admin is allowed to update any user");
}

#[test]
fn test_admin_list_allowed() {
    assert!(true, "TODO: admin is allowed to list users");
}

#[test]
fn test_user_list() {
    let state = DbState::new();
    state.clean_tables();
    state.create_user("inactive@domain.com", false);
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true);
    let api_base_uri = common::api_base_url();
    let client = Client::new();

    // query all users
    let mut response = client
        .get(api_base_uri.join("/users").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: ResponseListUser = response.json()
        .expect("Could not deserialize user list");
    assert_eq!(resp_data.users.len(), 2);

    // query only active users
    let mut response = client
        .get(api_base_uri.join("/users?active=true").unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: ResponseListUser = response.json()
        .expect("Could not deserialize user list");
    assert_eq!(resp_data.users.len(), 1);
    assert_eq!(resp_data.users[0].email, "josh@domain.com");
}

#[test]
fn test_user_login() {
    let dbstate = DbState::new();
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let (user_data, pass, _) = common::signup_user(&dbstate.conn, "josh@domain.com", true);
    let user_id = user_data.user.id;

    // login
    let login_data = json!({
        "password": pass,
        "email": user_data.user.email
    });
    let mut response = client
        .post(api_base_uri.join("login").unwrap())
        .json(&login_data)
        .send()
        .expect("Login failed");
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: ResponseLoginSignup = response.json().unwrap();
    assert_eq!(resp_data.user.id, user_id);
    assert_eq!(resp_data.user.is_active, true);
    assert_eq!(resp_data.user.role, "user");
}

#[test]
fn test_user_login_generate_auth_token() {
    let state = DbState::new();
    let client = Client::new();
    let (user_data, password, token) = common::signup_user(&state.conn, "josh@domain.com", true);
    let user_id = user_data.user.id;

    let login_data = json!({
        "password": password,
        "email": user_data.user.email
    });

    // login again, token returned should be different
    let resp_data = common::user_login(&client, &login_data, StatusCode::OK).unwrap();
    assert_eq!(resp_data.auth.user_id, user_id);
    assert_ne!(resp_data.auth.token, token);
}

#[test]
fn test_user_expire_auth_token() {
    let state = DbState::new();
    let (user, _) = state.create_user("user@domain.com", false);
    let client = Client::new();

    // create a bunch of tokens
    let expiry_date_far_expired = chrono::NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    let expiry_date_close = chrono::NaiveDateTime::from_timestamp(
        (chrono::Utc::now() + chrono::Duration::hours(1))
            .timestamp(), 0);
    let expiry_date_just_expired = chrono::NaiveDateTime::from_timestamp(
        (chrono::Utc::now() + chrono::Duration::hours(1) - chrono::Duration::seconds(1) )
            .timestamp(), 0);

    let mut auth = state.create_auth(user.id, &user.email, expiry_date_far_expired)
        .expect("Could not create auth token");
    common::get_user_detail(&client, user.id, auth.token, StatusCode::UNAUTHORIZED);
    auth = state.create_auth(user.id, &user.email, expiry_date_close)
        .expect("Could not create auth token");
    common::get_user_detail(&client, user.id, auth.token, StatusCode::OK);
    auth = state.create_auth(user.id, &user.email, expiry_date_just_expired)
        .expect("Counld not create auth token");
    common::get_user_detail(&client, user.id, auth.token, StatusCode::UNAUTHORIZED);
 }

#[test]
fn test_user_login_trim_expired_auth_token() {
    let state = DbState::new();
    let client = Client::new();
    let (user_data, password, _) = common::signup_user(&state.conn, "josh@domain.com", true);
    let user_id = user_data.user.id;

    // create an expired token
    let expiry_date_expired = chrono::NaiveDateTime::parse_from_str("2017-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    state.create_auth(user_id, &user_data.user.email, expiry_date_expired).unwrap();
    assert_eq!(2, state.count_auth_token(user_data.user.id));
    let login_data = json!({
        "password": password,
        "email": user_data.user.email
    });
    common::user_login(&client, &login_data, StatusCode::OK).unwrap();
    // (-1, +1) we removed the expired token and added the new one
    assert_eq!(2, state.count_auth_token(user_data.user.id));
}

#[test]
fn test_user_login_rotate_auth_token() {

    let state = DbState::new();
    state.clean_tables();
    let client = Client::new();
    let (user_data, password, first_token) = common::signup_user(&state.conn, "josh@domain.com", true);
    assert_eq!(state.count_auth_token(user_data.user.id), 1);

    // moar tokens generated
    let login_data = json!({
        "password": password,
        "email": user_data.user.email
    });
    let resp_data = common::user_login(&client, &login_data, StatusCode::OK).unwrap();
    assert_ne!(first_token, resp_data.auth.token);
    common::user_login(&client, &login_data, StatusCode::OK).unwrap();

    for _ in 0..25 {
        common::user_login(&client, &login_data, StatusCode::OK).unwrap();
    }
    assert_eq!(state.count_auth_token(user_data.user.id), open_taffeta_lib::config::MAX_AUTH_TOKEN);
}
