#![allow(proc_macro_derive_resolution_fallback)]

use std::io;

use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FormDataError, FormError};
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::json;
use rocket_contrib::templates::Template;

use serde_derive::Serialize;

use log::{debug, error, warn};

use crate::auth::cookie::AdminCookie;
use crate::config;
use crate::crypto;
use crate::db;
use crate::models::{Role, User, ROLE_ADMIN};
use crate::responses::{created, APIResponse};

#[derive(Serialize)]
struct TemplateAdminContext<'a> {
    title: &'a str,
    users: Vec<(User, Role)>,
    message: &'a str,
    current_user_id: i32,
}

#[derive(Debug, FromForm)]
pub struct FormEditUser {
    user_id: i32,
    is_active: bool,
}

#[derive(Debug, FromForm)]
pub struct FormLogin {
    email: String,
    password: String,
}

fn generate_template(
    title: &str,
    user_role_list: Vec<(User, Role)>,
    user_id: i32,
    message: &str,
) -> Template {
    let ctx = TemplateAdminContext {
        title: title,
        users: user_role_list,
        message: message,
        current_user_id: user_id,
    };
    Template::render("admin_users_list", &ctx)
}

#[get("/admin")]
pub fn admin_panel_get_login(_admin: AdminCookie) -> Redirect {
    Redirect::to("/admin/users")
}

#[get("/admin", rank = 2)]
pub fn admin_panel_get_login_noauth() -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[post("/admin", data = "<sink>")]
pub fn admin_panel_post_login(
    conn: db::Conn,
    sink: Result<Form<FormLogin>, FormError>,
    mut cookies: Cookies,
) -> Redirect {
    let mut retval = Redirect::to("/admin");
    match sink {
        Ok(form) => {
            let hashed_pwd = crypto::hash_password(form.password.as_bytes());
            match db::get_active_user(&conn, &hashed_pwd, &form.email) {
                Ok((user, role)) => {
                    if role.name == ROLE_ADMIN {
                        // TODO: create unique cookie content
                        let cookie_value = format!("{}:{}", user.id, role.name);
                        let cookie = Cookie::new(config::COOKIE_NAME_AUTH_STATUS, cookie_value);
                        // TODO: how does building a cookie work?
                        // let cookie_b = Cookie::build("auth_status_pvt", "OK")
                        //     .path("/admin")
                        //     .secure(true)
                        //     .finish();
                        cookies.add_private(cookie);
                        retval = Redirect::to("/admin/users");
                    }
                }
                Err(_) => {
                    // error!(">>> query failed {:?}", err);
                }
            };
        }
        Err(FormDataError::Io(_)) => {
            error!("Form input was invalid UTF-8.");
        }
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            error!("Invalid form input: {}", f);
        }
    }
    retval
}

#[get("/admin/users")]
pub fn admin_panel(conn: db::Conn, admin: AdminCookie) -> Template {
    let users = db::get_user_list(&conn, false).expect("Could not get users list");
    debug!("Found {} users", users.len());
    generate_template("User List", users, admin.user_id, "Click a checkbox to update users")
}

#[get("/admin/users", rank = 2)]
pub fn admin_panel_redirect() -> Redirect {
    // "Sorry, you must be an administrator to access this page."
    Redirect::to("/admin")
}

#[post("/admin/edit_user", data = "<user_data>")]
pub fn admin_panel_edit_user(
    conn: db::Conn,
    user_data: Result<Form<FormEditUser>, FormError>,
    _admin: AdminCookie,
) -> APIResponse {
    let msg: &str;
    match user_data {
        Ok(form) => {
            let mut user = db::get_user(&conn, form.user_id)
                .expect(&format!("Could not retrieve user from form data {:?}", form));
            user.is_active = form.is_active;
            match db::update_user(&conn, &user) {
                Ok(_) => msg = "User updated successfully",
                Err(_) => msg = "User update failed",
            };
        }
        Err(FormDataError::Io(_)) => {
            msg = "Form edit user has invalid UTF-8";
            warn!("{}", msg);
        }
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            warn!("Invalid form edit user received: {}", f);
            msg = "Invalid form edit user received";
        }
    }
    created().data(json!({ "detail": msg }))
}
