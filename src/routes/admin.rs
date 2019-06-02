#![allow(proc_macro_derive_resolution_fallback)]

use crate::models::User;
use crate::db;
use crate::auth::token::Auth;
use rocket::response::Redirect;

#[get("/admin")]
pub fn admin_panel(_conn: db::Conn, _auth: Auth, admin: User) -> &'static str {
    eprintln!("Pass through for admin {:?}", admin.email);
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
pub fn admin_panel_user(_conn: db::Conn, _auth: Auth, user: User) -> &'static str {
    eprintln!("Pass through for user {:?}", user.email);
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
pub fn admin_panel_redirect() -> Redirect {
    eprintln!("Pass through straight to login");
    Redirect::to("/login")
}
