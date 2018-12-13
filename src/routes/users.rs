#[allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::User;
use crate::responses::{bad_request, created, APIResponse};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;
use rocket::data::{self, FromDataSimple};
use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::http::Status;
use rocket::response::status;
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

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    #[validate(
        length(min = "6", message = "Password too short"),
        custom = "validate_pwd_strength"
    )]
    password: String,
    #[validate(email(message = "Invalid email"))]
    email: String,
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

#[get("/users", format = "application/json")]
pub fn get_users(conn: db::Conn, auth: Auth) -> JsonValue {
    let users_rs = users.load::<User>(&*conn).expect("error retrieving users");
    json!({ "users": users_rs })
}

#[get("/user/<user_id>", format = "application/json")]
pub fn get_user(conn: db::Conn, auth: Auth, user_id: i32) -> JsonValue {
    let user: Vec<User> = users
        .filter(active.eq(true))
        .filter(id.eq(user_id))
        .load(&*conn)
        .expect(&format!("error retrieving user id={}", user_id));

    if user.len() != 1 {
        let resp_data = json!({
            "status": "error",
            "detail": format!("Wrong records found ({}) for user_id={}",
                              user.len(), user_id)
        });
        bad_request().data(resp_data);
    }
    json!({ "user": user[0] })
}

#[post("/signup", data = "<user_data>", format = "application/json")]
pub fn signup_user(conn: db::Conn, user_data: Json<NewUser>) -> APIResponse {
    let new_user = NewUser {
        username: user_data.username.clone(),
        password: user_data.password.clone(),
        email: user_data.email.clone(),
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

    match diesel::insert_into(users).values(&new_user).execute(&*conn) {
        Err(err) => {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
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
                .filter(username.eq(&new_user.username))
                .first(&*conn)
                .expect(&format!(
                    "error getting user with username {}",
                    new_user.username
                ));

            let user_auth = user.to_user_auth();
            let resp_data = json!({ "user": user_auth });
            created().data(resp_data)
        }
    }
}
