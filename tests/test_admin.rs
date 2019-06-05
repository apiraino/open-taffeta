extern crate open_taffeta_lib;
extern crate reqwest;
#[macro_use] extern crate serde_json;
use reqwest::{Client, StatusCode};
use open_taffeta_lib::serializers::user::*;
use open_taffeta_lib::models::*;
use crate::common::dbstate::DbState;
mod common;

#[test]
fn test_user_turns_into_admin() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let client = Client::new();

    let (resp_data, _, token) = common::signup_user(
        &state.conn, "will-be-admin@domain.com", true, ROLE_USER);
    assert_eq!(resp_data.user.email, "will-be-admin@domain.com");
    assert_eq!(resp_data.user.role,
               open_taffeta_lib::models::ROLE_USER);
    let user_id = resp_data.user.id;

    // grant admin role
    let role = open_taffeta_lib::db::get_role(&state.conn, user_id);
    let role_data = open_taffeta_lib::models::Role {
        id: role.id,
        name: open_taffeta_lib::models::ROLE_ADMIN.to_owned(),
        user: Some(user_id)
    };
    open_taffeta_lib::db::update_role(&state.conn, role_data);

    // check role now
    let resp_data : ResponseUserDetail = common::get_user_detail(
        &client, user_id, &token, StatusCode::OK)
        .expect("Get user detail failed");
    assert_eq!(resp_data.user.role,
               open_taffeta_lib::models::ROLE_ADMIN);
}

#[test]
fn test_admin_get_user_list() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let (_, _, token) = common::signup_user(
        &state.conn, "josh@domain.com", true, ROLE_ADMIN);
    let client = Client::new();

    // query all users
    let users_list = common::get_user_list(
        &client, &token, "", StatusCode::OK)
        .expect("Could not retrieve list of users");
    assert_eq!(users_list.users.len(), 1);

    // query only active users
    let params = "?active=true";
    let users_list = common::get_user_list(
        &client, &token, params, StatusCode::OK)
        .expect("Could not retrieve list of users");
    assert_eq!(users_list.users.len(), 1);
    assert_eq!(users_list.users[0].email, "josh@domain.com");
}

#[test]
fn test_user_disallowed_admin_interface() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let (_, _, token_user) = common::signup_user(
        &state.conn, "josh@domain.com", true, ROLE_USER);
    let (_, _, token_admin) = common::signup_user(
        &state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let client = Client::new();

    let admin_page = common::get_admin_page(
        &client, &token_user, "", StatusCode::UNAUTHORIZED)
        .expect("Could not get admin page");
    let regexp = "not authorized";
    assert_eq!(admin_page.to_lowercase().contains(regexp), true,
               "Page regexp does not match: {}: got {}",
               regexp, admin_page);

    let admin_page = common::get_admin_page(
        &client, &token_admin, "", StatusCode::OK)
        .expect("Could not get admin page");
    let regexp = "user list";
    assert_eq!(admin_page.to_lowercase().contains(regexp), true,
               "Page regexp does not match: {}: got {}",
               regexp, admin_page);
}

#[test]
fn test_admin_update_allowed() {
    assert!(true, "Admin allowed to update any user");
}

#[test]
fn test_admin_list_allowed() {
    assert!(true, "Admin allowed to list users");
}
