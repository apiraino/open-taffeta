#![allow(proc_macro_derive_resolution_fallback)]

use crate::models::{Role, User};
use crate::db;
use crate::auth::token::Auth;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

#[derive(Serialize)]
struct TemplateAdminContext<'a> {
    title: &'a str,
    users: Vec<(User,Role)>
}

fn generate_html(title: &str, user_role_list: Vec<(User,Role)>) -> Template {
    // let rocket = rocket::ignite().attach(Template::fairing());
    // let client = Client::new(rocket).expect("valid rocket");
    let ctx = TemplateAdminContext {
        title: title,
        users: user_role_list
    };
    Template::render("admin_users_list", &ctx)
}

#[get("/admin")]
pub fn admin_panel(conn: db::Conn, _auth: Auth, _admin: User) -> Template {
    let users = db::get_user_list(&conn, true)
        .expect("Could not get users list");
    generate_html("User List", users)
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
