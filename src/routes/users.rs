#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

use rocket_contrib::json;
// https://mozilla.logbot.info/rocket/20181211#c15708806
// - Json<T> does not change anywhere.
// - Json<Value> as a responder changes to JsonValue
// - The new JsonValue is only really interesting as a Responder
use rocket_contrib::json::Json;

use validator::{Validate, ValidationError};
use validator_derive::Validate;

use serde_derive::{Deserialize, Serialize};

use crate::auth::admin::AdminUser;
use crate::auth::token::{self as auth, Auth};
use crate::crypto;
use crate::db;
use crate::models::{self as models, Role, RoleNew, User};
use crate::responses::*;
use crate::schema::users::dsl::*;
use crate::schema::{roles, users};
use crate::serializers::user::{ResponseLoginSignup, UserBaseResponse, UserEdit};
use crate::utils;

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "users"]
pub struct UserLoginSignup<'a> {
    #[validate(
        length(min = "6", message = "Password too short"),
        custom = "validate_pwd_strength"
    )]
    password: &'a str,
    #[validate(email(message = "Invalid email"))]
    email: &'a str,
}

fn validate_pwd_strength(pwd: &str) -> Result<(), ValidationError> {
    if pwd == "123456" {
        // Constructor with `code` param
        // ValidationError::new("password complexity");

        let e = ValidationError {
            code: std::borrow::Cow::from("password complexity"),
            message: Some(std::borrow::Cow::from("Are you kidding me?")),
            params: std::collections::HashMap::new(),
        };

        // want another param?
        // e.add_param(std::borrow::Cow::from("param_name"), &"param_value");
        return Err(e);
    }
    Ok(())
}

#[get("/users?<active>", format = "application/json")]
pub fn get_users(
    conn: db::Conn,
    _auth: Auth,
    _admin: AdminUser,
    active: Option<bool>,
) -> APIResponse {
    let users_rs: Vec<(User, Role)> = match active {
        Some(_) => users::table
            .inner_join(roles::table)
            .filter(users::is_active.eq(true))
            .load(&*conn)
            .expect("error retrieving active users"),
        None => users::table.inner_join(roles::table).load(&*conn).expect("error retrieving users"),
    };

    let payload: Vec<UserBaseResponse> =
        users_rs.into_iter().map(|(user, role)| utils::attach_role_to_user(&user, &role)).collect();
    ok().data(json!({ "users": payload }))
}

#[get("/users/<user_id>", format = "application/json")]
pub fn get_user(conn: db::Conn, _auth: Auth, user_id: i32) -> APIResponse {
    let query_result: Result<(User, Role), diesel::result::Error> =
        users::table.inner_join(roles::table).filter(users::id.eq(user_id)).get_result(&*conn);

    match query_result {
        Err(diesel::NotFound) => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("error retrieving active user id {}", user_id)
            });
            bad_request().data(resp_data)
        }
        Err(err) => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("error executing query: {}", err)
            });
            bad_request().data(resp_data)
        }
        Ok((user, role)) => ok().data(json!({ "user": utils::attach_role_to_user(&user, &role) })),
    }
}

#[post("/login", data = "<user_data>", format = "application/json")]
pub fn login_user(conn: db::Conn, user_data: Json<UserLoginSignup>) -> APIResponse {
    let mut logmein = UserLoginSignup { password: user_data.password, email: user_data.email };

    // examples
    // https://github.com/ayourtch/diesel-join-example/blob/master/src/lib.rs
    // TODO: use `.select((tbl1::fld1, tbl2::fld2))` to get only some fields
    // let hashed_pwd = crypto::hash_password(user_data.password.as_bytes());
    let hashed_pwd = crypto::hash_password(user_data.password.as_bytes());
    logmein.password = &hashed_pwd;

    // TODO: refactor all this like in the admin login
    match db::get_user_profile(&conn, &logmein.email) {
        Ok(user) => {
            if user.is_active == false {
                let resp_data = json!({
                    "status": "error",
                    "detail": format!("Could not find user active for email {}", logmein.email)
                });
                // TODO: this 401 could simply return no body
                return unauthorized().data(json!(resp_data));
            }
        }
        Err(err) => {
            eprintln!(
                "{}",
                &format!("Could not retrieve valid user with email {:?}: {}", logmein.email, err)
            );
            let resp_data = json!({
                "status": "error",
                "detail": format!("Could not find user for email {}", logmein.email)
            });
            // TODO: this 401 could simply return no body
            return unauthorized().data(json!(resp_data));
        }
    };

    let user_rs: Vec<(User, Role)> = users::table
        .inner_join(roles::table)
        .filter(users::is_active.eq(true))
        .filter(users::email.eq(&logmein.email))
        .filter(users::password.eq(&logmein.password))
        .load(&*conn)
        .expect(&format!("error retrieving active user with email={}", logmein.email));

    match user_rs.len() {
        1 => {
            let user_auth = user_rs[0].0.to_auth();
            let user = &user_rs[0].0;
            let user_role = &user_rs[0].1;
            if let Err(err) = auth::save_auth_token(conn, &user_auth) {
                let resp_data = json!({
                    "status": "error",
                    "detail": format!("Failed to save new auth token for email {}: {}",
                                      logmein.email, err)
                });
                return bad_request().data(resp_data);
            }
            let user_data = utils::attach_role_to_user(&user, &user_role);
            let resp_data = ResponseLoginSignup { auth: user_auth, user: user_data };
            ok().data(json!(resp_data))
        }
        _ => {
            eprintln!(
                "{}",
                format!(
                    "Wrong records count found ({}) for email={}",
                    user_rs.len(),
                    logmein.email
                )
            );
            let resp_data = json!({
                "status": "error",
                "detail": format!("Could not find user for email {}", logmein.email)
            });
            // TODO: this 401 could simply return no body
            unauthorized().data(json!(resp_data))
        }
    }
}

#[post("/signup", data = "<user_data>", format = "application/json")]
pub fn signup_user(conn: db::Conn, user_data: Json<UserLoginSignup>) -> APIResponse {
    let mut new_user = UserLoginSignup { password: user_data.password, email: user_data.email };

    let res = new_user.validate();
    if res.is_err() {
        let errs = res.unwrap_err();
        let err_msg = format!("Data validation error: {:#?}", errs);
        let resp_data = json!({
            "status":"error",
            "detail": err_msg
        });
        return bad_request().data(resp_data);
    }
    // make the borrow check happy
    let hashed_pwd = crypto::hash_password(user_data.password.as_bytes());
    new_user.password = &hashed_pwd;
    match diesel::insert_into(users).values(&new_user).execute(&*conn) {
        Err(err) => {
            if let diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err
            {
                let resp_data = json!({
                    "status":"error",
                    "detail": format!("DB error - record already exists: {:?}", err)
                });
                bad_request().data(resp_data)
            } else {
                let resp_data = json!({
                    "status":"error",
                    "detail": format!("DB error: {:?}", err)
                });
                bad_request().data(resp_data)
            }
        }
        Ok(_) => {
            let user: User = users::table
                .filter(users::email.eq(&new_user.email))
                .first(&*conn)
                .expect(&format!("error getting user with email {}", new_user.email));

            let role_data = RoleNew { name: models::ROLE_USER.to_owned(), user: Some(user.id) };
            let user_role = db::add_role(&conn, role_data)
                .expect(&format!("Could not add/retrieve role for user {}", user.id));
            let user_auth = user.to_auth();
            if let Err(err) = auth::save_auth_token(conn, &user_auth) {
                let resp_data = json!({
                    "status": "error",
                    "detail": format!(
                        "Failed to save new auth token for email {}: {}",
                        user.email, err)
                });
                return bad_request().data(resp_data);
            }
            let user_data = utils::attach_role_to_user(&user, &user_role);
            let resp_data = ResponseLoginSignup { auth: user_auth, user: user_data };
            created().data(json!(resp_data))
        }
    }
}

#[put("/user/<user_id>", data = "<user_data>", format = "application/json")]
pub fn edit_user(
    conn: db::Conn,
    _auth: Auth,
    _user: User,
    user_id: i32,
    user_data: Json<UserEdit>,
) -> APIResponse {
    let mut user = db::get_user(&conn, user_id)
        .expect(&format!("Could not retrieve user from data {:?}", user_data));
    user.email = user_data.email.clone();
    match db::update_user(&conn, &user) {
        Err(err) => {
            let msg = format!("Error updating user {}: {}", user_id, err);
            let resp_data = json!({
                "status":"error",
                "detail": msg
            });
            return bad_request().data(resp_data);
        }
        Ok(_) => {
            eprintln!("Profile update successful for user {}", user_id);
        }
    }
    no_content()
}
