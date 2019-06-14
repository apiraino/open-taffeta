extern crate open_taffeta_lib;
extern crate reqwest;
// #[macro_use]
// extern crate serde_json;

use open_taffeta_lib::models::*;
use open_taffeta_lib::serializers::user::*;
use reqwest::{Client, StatusCode};

use cratetests::common;
use cratetests::common::dbstate::DbState;

#[test]
fn test_user_turns_into_admin() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let client = Client::new();

    let (resp_data, _, token) =
        common::signup_user(&state.conn, "will-be-admin@domain.com", true, ROLE_USER);
    assert_eq!(resp_data.user.email, "will-be-admin@domain.com");
    assert_eq!(resp_data.user.role, open_taffeta_lib::models::ROLE_USER);
    let user_id = resp_data.user.id;

    // grant admin role
    let role = open_taffeta_lib::db::get_role(&state.conn, user_id);
    let role_data = open_taffeta_lib::models::Role {
        id: role.id,
        name: open_taffeta_lib::models::ROLE_ADMIN.to_owned(),
        user: Some(user_id),
    };
    match open_taffeta_lib::db::update_role(&state.conn, role_data) {
        Err(err) => {
            panic!(err);
        }
        Ok(_) => {}
    };

    // check role now
    let resp_data: ResponseUserDetail =
        common::get_user_detail(&client, user_id, &token, StatusCode::OK)
            .expect("Get user detail failed");
    assert_eq!(resp_data.user.role, open_taffeta_lib::models::ROLE_ADMIN);
}

#[test]
fn test_admin_get_user_list() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let (_, _, token) = common::signup_user(&state.conn, "josh@domain.com", true, ROLE_ADMIN);
    let client = Client::new();

    // query all users
    let users_list = common::get_user_list(&client, &token, "", StatusCode::OK)
        .expect("Could not retrieve list of users");
    assert_eq!(users_list.users.len(), 1);

    // query only active users
    let params = "?active=true";
    let users_list = common::get_user_list(&client, &token, params, StatusCode::OK)
        .expect("Could not retrieve list of users");
    assert_eq!(users_list.users.len(), 1);
    assert_eq!(users_list.users[0].email, "josh@domain.com");
}

#[test]
fn test_admin_failed_login() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    common::signup_user(&state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let res = common::admin_login("admin@domain.com", "hey", StatusCode::UNAUTHORIZED);
    assert_eq!(true, res.is_err(), "{:?}", res);
    let err_msg = res.err().unwrap();
    let regexp = "redirect";
    assert_eq!(
        err_msg.contains(regexp),
        true,
        "Page regexp does not match: {}: got {:?}",
        regexp,
        err_msg
    );
}

#[test]
fn test_admin_login() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let (_, pass, _) = common::signup_user(&state.conn, "admin@domain.com", true, ROLE_ADMIN);
    let client_admin = common::admin_login("admin@domain.com", &pass, StatusCode::OK)
        .expect("Could not login into admin page");

    // admin get the users list
    let admin_page =
        common::get_admin_page(&client_admin, StatusCode::OK).expect("Could not get admin page");
    let regexp = "user list";
    assert_eq!(
        admin_page.to_lowercase().contains(regexp),
        true,
        "Page regexp does not match: {}: got {}",
        regexp,
        admin_page
    );
}

#[test]
fn test_admin_login_user() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let (_, pass, _) = common::signup_user(&state.conn, "user@domain.com", true, ROLE_USER);
    let res = common::admin_login("user@domain.com", &pass, StatusCode::OK);
    assert_eq!(true, res.is_err());
}
