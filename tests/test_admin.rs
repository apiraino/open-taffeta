extern crate open_taffeta_lib;

extern crate reqwest;

#[macro_use] extern crate serde_json;

// use reqwest::header::{AUTHORIZATION, HeaderValue};
use reqwest::{Client, StatusCode};

mod common;

use crate::common::dbstate::DbState;
use open_taffeta_lib::serializers::users::ResponseUserDetail;

#[test]
fn test_user_turns_into_admin() {
    let state = DbState::new();
    state.clean_tables();
    state.assert_empty_users();
    let client = Client::new();

    let (resp_data, _, token) = common::signup_user(
        &state.conn, "wanna-be-admin@domain.com", true);
    assert_eq!(resp_data.user.email, "wanna-be-admin@domain.com");
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
        &client, user_id, token, StatusCode::OK)
        .expect("Get user detail failed");
    assert_eq!(resp_data.user.role,
               open_taffeta_lib::models::ROLE_ADMIN);
}
