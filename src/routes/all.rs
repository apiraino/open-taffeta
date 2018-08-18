#[allow(proc_macro_derive_resolution_fallback)]
use rocket_contrib::{Json, Value};

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[get("/")]
fn get_index() -> &'static str {
    "Welcome!"
}
