#![allow(proc_macro_derive_resolution_fallback)]
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "detail": "Resource was not found."
    })
}

#[catch(401)]
pub fn not_authorized() -> JsonValue {
    json!({
        "status": "error",
        "detail": "Not authorized"
    })
}

#[get("/")]
pub fn get_index() -> &'static str {
    "Hey there! Interested in Rust?\n\n
Come stop by the LuXeria place!\n\n
Endpoints: https://github.com/apiraino/open-taffeta/wiki/Endpoints"
}
