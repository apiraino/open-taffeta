extern crate open_taffeta_lib;

#[macro_use]
extern crate dotenv_codegen;

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

// #[test]
// fn test_bad_auth() {
//     common::setup();
//     let api_base_uri = common::api_base_url();
//     let client = Client::new();
//     let response = client
//         .post(api_base_uri.join("/doors").unwrap())
//         .body("{'name':'door123'}")
//         .header(Authorization(auth_token))
//         .send()
//         .unwrap();
//     assert_bad_request_response(response);
//     common::teardown();
// }
