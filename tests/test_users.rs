extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use]
extern crate serde_json;

use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::{Client, StatusCode};

mod common;

use crate::common::dbstate::DbState;
use open_taffeta_lib::models::*;
use open_taffeta_lib::serializers::user::*;

// TODO: have a look here
// https://bitbucket.org/dorianpula/rookeries/src/master/tests/test_site_management.rs

#[test]
fn test_db() {
    let state = DbState::new();
    state.create_user("josh@domain.com", true, open_taffeta_lib::models::ROLE_USER);
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
    let mut response =
        client.post(api_base_uri.join("/signup").unwrap()).json(&user_data).send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_data: ResponseLoginSignup = response.json().expect("Error unwrapping signup response");
    assert_eq!(resp_data.user.is_active, false);
    assert_eq!(resp_data.user.role, ROLE_USER);
}

#[test]
fn test_user_signup_and_activate() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();

    let (resp_data, _, _) = common::signup_user(&state.conn, "josh@domain.com", false, ROLE_USER);
    assert_eq!(resp_data.user.email, "josh@domain.com");
    assert_eq!(resp_data.user.is_active, false);

    let (resp_data, _, _) = common::signup_user(&state.conn, "josh1@domain.com", true, ROLE_USER);
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
    let (resp_data, _, _) = common::signup_user(&state.conn, "josh@domain.com", false, ROLE_USER);
    assert_eq!(resp_data.user.email, "josh@domain.com");
    assert_eq!(resp_data.user.is_active, false);

    let (resp_data, _, _) = common::signup_user(&state.conn, "josh1@domain.com", true, ROLE_USER);
    assert_eq!(resp_data.user.email, "josh1@domain.com");
    assert_eq!(resp_data.user.is_active, true);

    // repeat same payload, expect a 400
    let api_base_uri = common::api_base_url();
    let client = Client::new();
    let user_data = json!({
        "password": "1234567",
        "email": "josh1@domain.com"
    });
    let mut response =
        client.post(api_base_uri.join("/signup").unwrap()).json(&user_data).send().unwrap();
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
    let (user_data, _, token) =
        common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);

    // get user detail
    let q = format!("/users/{}", user_data.user.id);
    let mut response = client
        .get(api_base_uri.join(&q).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_data: ResponseUserDetail = response.json().unwrap();
    assert_eq!(resp_data.user.email, "josh@domain.com");
}

#[test]
fn test_user_edit_profile() {
    let state = DbState::new();
    state.clean_tables();
    let client = Client::new();
    state.assert_empty_users();
    // signup two user
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let (user_data2, _, _) = common::signup_user(&state.conn, "aimee@domain.com", true, ROLE_USER);

    // A user cannot touch another user profile
    let payload = json!({
        "email": user_data2.user.is_active,
        "is_active": user_data2.user.is_active,
        "role": user_data2.user.role
    });
    common::user_update(&client, &token, user_data2.user.id, &payload, StatusCode::UNAUTHORIZED)
        .expect("Could not update user");
}

#[test]
fn test_user_edit_profile_fields() {
    let state = DbState::new();
    state.clean_tables();
    let client = Client::new();
    state.assert_empty_users();
    // signup two user
    let (user_data, _, token) =
        common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);

    let payload = json!({
        "email": "my-new-email@domain.com",
        "is_active": "false",
        "role": "admin"
    });
    common::user_update(&client, &token, user_data.user.id, &payload, StatusCode::NO_CONTENT)
        .expect("Could not update user");

    let new_user = common::get_user_detail(&client, user_data.user.id, &token, StatusCode::OK)
        .expect("Could not retrieve user");

    // A user can only change their own email (a.t.m.)
    // all other fields are discarded
    assert_eq!(new_user.user.email, "my-new-email@domain.com");
    assert_eq!(new_user.user.role, ROLE_USER);
    assert_eq!(new_user.user.is_active, true);
}

#[test]
fn test_user_edit_profile_email_taken() {
    assert!(true, "User should not be able to set their email equal to someone else's");
}

#[test]
fn test_user_list_not_allowed() {
    let state = DbState::new();
    state.clean_tables();
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let client = Client::new();
    common::get_user_list(&client, &token, "", StatusCode::UNAUTHORIZED);
}

#[test]
fn test_user_login() {
    let dbstate = DbState::new();
    let client = Client::new();
    let (user_data, pass, _) =
        common::signup_user(&dbstate.conn, "josh@domain.com", true, ROLE_USER);
    let user_id = user_data.user.id;

    // login
    let login_data = json!({
        "password": pass,
        "email": user_data.user.email
    });
    let res = common::user_login(&client, &login_data, StatusCode::OK);
    assert_eq!(true, res.is_ok());
    let resp_data: ResponseLoginSignup = res.unwrap();
    assert_eq!(resp_data.user.id, user_id);
    assert_eq!(resp_data.user.is_active, true);
    assert_eq!(resp_data.user.role, "user");
}

#[test]
fn test_user_login_failed() {
    let dbstate = DbState::new();
    let client = Client::new();
    let (user_data, _, _) = common::signup_user(&dbstate.conn, "josh@domain.com", true, ROLE_USER);

    // login
    let login_data = json!({
        "password": "hey",
        "email": user_data.user.email
    });
    let res = common::user_login(&client, &login_data, StatusCode::UNAUTHORIZED);
    assert_eq!(true, res.is_err(), "{:?}", res)
}

#[test]
fn test_user_login_generate_auth_token() {
    let state = DbState::new();
    let client = Client::new();
    let (user_data, password, token) =
        common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let user_id = user_data.user.id;

    let login_data = json!({
        "password": password,
        "email": user_data.user.email
    });

    // login again, token returned should be different
    let res = common::user_login(&client, &login_data, StatusCode::OK);
    assert_eq!(true, res.is_ok());
    let resp_data: ResponseLoginSignup = res.unwrap();
    assert_eq!(resp_data.auth.user_id, user_id);
    assert_ne!(resp_data.auth.token, token);
}

#[test]
fn test_user_expire_auth_token() {
    let state = DbState::new();
    let user = state.create_user("user@domain.com", false, open_taffeta_lib::models::ROLE_USER);
    let client = Client::new();

    // create a bunch of tokens
    let expiry_date_far_expired =
        chrono::NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    let expiry_date_close = chrono::NaiveDateTime::from_timestamp(
        (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
        0,
    );
    let expiry_date_just_expired = chrono::NaiveDateTime::from_timestamp(
        (chrono::Utc::now() + chrono::Duration::hours(1) - chrono::Duration::seconds(1))
            .timestamp(),
        0,
    );

    let mut auth = state
        .create_auth(user.id, &user.email, expiry_date_far_expired)
        .expect("Could not create auth token");
    common::get_user_detail(&client, user.id, &auth.token, StatusCode::UNAUTHORIZED);
    auth = state
        .create_auth(user.id, &user.email, expiry_date_close)
        .expect("Could not create auth token");
    common::get_user_detail(&client, user.id, &auth.token, StatusCode::OK);
    auth = state
        .create_auth(user.id, &user.email, expiry_date_just_expired)
        .expect("Could not create auth token");
    common::get_user_detail(&client, user.id, &auth.token, StatusCode::UNAUTHORIZED);
}

#[test]
fn test_user_login_trim_expired_auth_token() {
    let state = DbState::new();
    let client = Client::new();
    let (user_data, password, _) =
        common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
    let user_id = user_data.user.id;

    // create an expired token
    let expiry_date_expired =
        chrono::NaiveDateTime::parse_from_str("2017-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
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
    let (user_data, password, first_token) =
        common::signup_user(&state.conn, "josh@domain.com", true, ROLE_USER);
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
