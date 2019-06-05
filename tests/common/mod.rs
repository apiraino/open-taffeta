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

use diesel::sqlite::SqliteConnection;

use open_taffeta_lib::serializers::user::*;
use open_taffeta_lib::models::*;

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

pub fn signup_user(conn: &SqliteConnection, email: &str, is_active: bool, role: &str) ->
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
        .expect("Could not signup user");

    if response.status() != StatusCode::CREATED {
        let r : ResponseError = response.json()
            .expect("Error opening signup response");
        let err_msg = format!(
            "Error in signup: expected {}, got {}: {:?}",
            StatusCode::CREATED, response.status(),
            r.detail
        );
        panic!(err_msg);
    }

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

    if role != ROLE_USER {
        let mut user_role = open_taffeta_lib::db::get_role(
            &conn, resp_data.auth.user_id);
        user_role.name = String::from(role);
        open_taffeta_lib::db::update_role(&conn, user_role);
    }

    // get back that user (Sqlite has no RETURNING support)
    let resp_data = get_user_detail(&client, resp_data.auth.user_id,
                                    &token, StatusCode::OK)
        .expect("no response received");

    (resp_data, user_data.get("password").unwrap().to_string(), token)
}

pub fn get_user_detail(client: &Client, user_id: i32, auth_token: &str, expected_status_code: StatusCode) -> Option<ResponseUserDetail> {
    let api_base_uri = api_base_url();
    let mut response = client
        .get(api_base_uri.join(&format!("/users/{}", user_id)).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(auth_token).unwrap())
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

pub fn get_user_list(client: &Client, token: &str, params: &str, expected_status_code: StatusCode) -> Option<ResponseListUser> {
    let api_base_uri = api_base_url();

    let mut url = String::from("/users");
    if params != "" {
        url.push_str(params);
    }

    let mut response = client
        .get(api_base_uri.join(&url.to_owned()).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token).unwrap())
        .send()
        .expect("Failed request: user list");
    if response.status() != expected_status_code {
        let err_msg;
        if response.status() != StatusCode::OK {
            eprintln!("{:?}", response);
            let r : ResponseError = response.json()
                .expect("Error opening user list response");
            err_msg = format!(
                "Error in get user list: expected {}, got {}: {:?}",
                expected_status_code, response.status(),
                r.detail);
        } else {
            err_msg = format!(
                "Error in get user list: expected {}, got {}",
                expected_status_code, response.status());
        }
        panic!(err_msg);
    }
    if response.status() == StatusCode::OK {
        let r : ResponseListUser = response.json()
            .expect("Error opening user list response");
        return Some(r);
    }
    None
}

pub fn user_login(client: &Client, login_data: &Value, expected_status_code: StatusCode)
                  -> Option<ResponseLoginSignup> {
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

pub fn get_admin_page(client: &Client, token: &str, params: &str, expected_status_code: StatusCode)
                      -> Option<String> {
    let api_base_uri = api_base_url();

    let mut url = String::from("/admin");
    if params != "" {
        url.push_str(params);
    }

    let mut response = client
        .get(api_base_uri.join(&url.to_owned()).unwrap())
        .header(AUTHORIZATION, HeaderValue::from_str(token).unwrap())
        .send()
        .expect("Failed request: admin page");
    if response.status() != expected_status_code {
        let err_msg;
        if response.status() != StatusCode::OK {
            eprintln!("{:?}", response);
            let r : ResponseError = response.json()
                .expect("Error opening admin page response");
            err_msg = format!(
                "Error in get admin page: expected {}, got {}: {:?}",
                expected_status_code, response.status(),
                r.detail);
        } else {
            err_msg = format!(
                "Error in admin page: expected {}, got {}",
                expected_status_code, response.status());
        }
        panic!(err_msg);
    }
    // if response.status() == StatusCode::OK {
    //     // TODO: basic HTML parsing ?
    // }
    let r = response.text().expect("Failed to unwrap response text");
    Some(r)
}
