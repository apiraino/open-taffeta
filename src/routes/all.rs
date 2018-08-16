#[allow(proc_macro_derive_resolution_fallback)]
#[get("/")]
fn get_index() -> &'static str {
    "Welcome!"
}
