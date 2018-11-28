#[allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::User;
use crate::responses::{bad_request, created, APIResponse};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::{status, Failure};
use rocket_contrib::{Json, Value};
use validator::{Validate, ValidationError};

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
fn get_users(conn: db::Conn) -> Json<Value> {
    let users_rs = users.load::<User>(&*conn).expect("error retrieving users");
    Json(json!({ "users": users_rs }))
}

#[get("/user/<user_id>", format = "application/json")]
fn get_user(conn: db::Conn, user_id: i32) -> Json<Value> {
    let user: Vec<User> = users
        .filter(active.eq(true))
        .filter(id.eq(user_id))
        .load(&*conn)
        .expect(&format!("error retrieving user id={}", user_id));
    Json(json!({ "user": user }))
}

#[post("/signup", data = "<user_data>", format = "application/json")]
fn signup_user(conn: db::Conn, user_data: Json<NewUser>) -> APIResponse {
    let new_user = NewUser {
        username: user_data.username.clone(),
        password: user_data.password.clone(),
        email: user_data.email.clone(),
    };

    let res = new_user.validate();
    if res.is_err() {
        let errs = res.unwrap_err();
        let err_msg = format!("Data validation error: {:#?}", errs);
        println!("{:?}", err_msg);
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
