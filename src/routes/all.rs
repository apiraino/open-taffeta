#[allow(proc_macro_derive_resolution_fallback)]
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

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
