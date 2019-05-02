#![allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::User;
use crate::responses::{ok, bad_request, created, APIResponse};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use rocket_contrib::json;
// https://mozilla.logbot.info/rocket/20181211#c15708806
// - Json<T> does not change anywhere.
// - Json<Value> as a responder changes to JsonValue
// - The new JsonValue is only really interesting as a Responder
use rocket_contrib::json::{Json, JsonValue};
use validator::{Validate, ValidationError};
use validator_derive::Validate;
use serde_derive::{Serialize, Deserialize};
use crate::auth::Auth;
use crate::crypto;

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "users"]
pub struct UserLoginSignup {
    #[validate(
        length(min = "6", message = "Password too short"),
        custom = "validate_pwd_strength"
    )]
    password: String,
    #[validate(email(message = "Invalid email"))]
    email: String
}

pub fn validate_pwd_strength(pwd: &str) -> Result<(), ValidationError> {
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

#[get("/users?<is_active>", format = "application/json")]
pub fn get_users(conn: db::Conn, _auth: Auth, is_active: Option<bool>) -> JsonValue {
    let users_rs : Vec<User> = match is_active {
        Some(_) => {
            users
                .filter(active.eq(is_active.unwrap_or_else(|| false)))
                .load(&*conn)
                .expect("error retrieving users")
        },
        None => {
            users
                .load(&*conn)
                .expect("error retrieving users")
        }
    };
    json!({ "users": users_rs })
}

#[get("/users/<user_id>", format = "application/json")]
pub fn get_user(conn: db::Conn, _auth: Auth, user_id: i32) -> APIResponse {
    let user : Result<User, diesel::result::Error> = users
        // .filter(active.eq(true))
        .filter(id.eq(user_id))
        .get_result(&*conn);

    match user {
        Err(diesel::NotFound) => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("error retrieving active user id = {}", user_id)
            });
            bad_request().data(resp_data)
        },
        _ =>  {
            ok().data(json!({ "user": user.expect("Unmanaged error while unwrapping user entity") }))
        }
    }
}

#[post("/login", data = "<user_data>", format = "application/json")]
pub fn login_user(conn: db::Conn, user_data: Json<UserLoginSignup>) -> APIResponse {
    let logmein = UserLoginSignup {
        password: user_data.password.clone(),
        email: user_data.email.clone()
    };

    let err_msg = format!("error retrieving active user with email={}", logmein.email);
    let user: Vec<User> = users
        .filter(active.eq(true))
        .filter(email.eq(logmein.email.clone()))
        .load(&*conn)
        .expect(&err_msg);

    match user.len() {
        1 => {
            ok().data(json!({ "user": user[0].to_user_auth() }))
        },
        _ => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("Wrong records count found ({}) for email={}",
                                  user.len(), logmein.email)
            });
            bad_request().data(resp_data)
        }
    }
}

#[post("/signup", data = "<user_data>", format = "application/json")]
pub fn signup_user(conn: db::Conn, user_data: Json<UserLoginSignup>) -> APIResponse {
    let mut new_user = UserLoginSignup {
        password: user_data.password.clone(),
        email: user_data.email.clone()
    };

    let res = new_user.validate();
    if res.is_err() {
        let errs = res.unwrap_err();
        let err_msg = format!("Data validation error: {:#?}", errs);
        let resp_data = json!({
            "status":"error",
            "detail": err_msg
        });
        bad_request().data(resp_data);
    }
    new_user.password = crypto::hash_password(new_user.password);

    match diesel::insert_into(users).values(&new_user).execute(&*conn) {
        Err(err) => {
            if let diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            ) = err
            {
                let err_msg = format!("DB error - record already exists: {:?}", err);
                println!("{:?}", err_msg);
                let resp_data = json!({
                    "status":"error",
                    "detail": err_msg
                });
                bad_request().data(resp_data)
            } else {
                let err_msg = format!("DB error: {:?}", err);
                println!("{:?}", err_msg);
                let resp_data = json!({
                    "status":"error",
                    "detail": err_msg
                });
                bad_request().data(resp_data)
            }
        }
        Ok(_) => {
            let user: User = users
                .filter(email.eq(&new_user.email))
                .first(&*conn)
                .unwrap_or_else(|_| panic!("error getting user with email={}", new_user.email));

            let user_auth = user.to_user_auth();
            let resp_data = json!({ "user": user_auth });
            created().data(resp_data)
        }
    }
}
