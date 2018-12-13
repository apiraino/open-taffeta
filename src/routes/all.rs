use crate::responses::{bad_request, created, APIResponse};
#[allow(proc_macro_derive_resolution_fallback)]
use rocket::response::status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::Deserialize;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(401)]
pub fn not_authorized() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Not authorized."
    })
}

#[get("/")]
pub fn get_index() -> &'static str {
    "Welcome!"
}

// A couple of tests for the best strategy
// to manage responses

#[derive(Deserialize)]
pub struct Message {
    s: String,
}

fn make_response(
    payload: Result<JsonValue, JsonValue>,
) -> Result<status::Created<JsonValue>, status::BadRequest<JsonValue>> {
    match payload {
        Ok(res) => Ok(status::Created(
            format!("{host}:{port}", host = "localhost", port = 8000).to_string(),
            Some(res),
        )),
        Err(err) => Err(status::BadRequest(Some(err))),
    }
}

#[post(
    "/test_post",
    data = "<message>",
    format = "application/json"
)]
pub fn tester(
    message: Json<Message>,
) -> Result<status::Created<JsonValue>, status::BadRequest<JsonValue>> {
    if message.s == "hello" {
        let resp_data = json!({
                 "status":"ok",
                 "detail":"Hello to you!"});
        make_response(Ok(resp_data))
    } else {
        let resp_data = json!({
            "status": "error",
            "detail":"something went foobar"
        });
        make_response(Err(resp_data))
    }
}

#[post(
    "/test_post_2",
    data = "<message>",
    format = "application/json"
)]
pub fn tester_2(message: Json<Message>) -> Result<APIResponse, APIResponse> {
    if message.s == "hello" {
        let resp_data = json!({
            "status":"ok",
            "detail":"Hello to you!"});
        Ok(created().data(resp_data))
    } else {
        let resp_data = json!({
            "status": "error",
            "detail":"something went foobar"
        });
        Ok(bad_request().data(resp_data))
    }
}

#[post(
    "/test_post_3",
    data = "<message>",
    format = "application/json"
)]
pub fn tester_3(message: Json<Message>) -> APIResponse {
    if message.s == "hello" {
        let resp_data = json!({
            "status":"ok",
            "detail":"Hello to you!"});
        created().data(resp_data)
    } else {
        let resp_data = json!({
            "status": "error",
            "detail":"something went foobar"
        });
        bad_request().data(resp_data)
    }
}
