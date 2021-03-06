#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use log::debug;
use reqwest::{Client, Url};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::auth::admin::AdminUser;
use crate::auth::token::Auth;
use crate::db;
use crate::models::{Door, User};
use crate::schema::doors;

// If a module manages more tables, keep dsl imports in functions
// https://gitter.im/diesel-rs/diesel?at=5b74459749932d4fe4e690b8
use crate::crypto::calculate_hash;
use crate::responses::{bad_request, created, no_content, ok, APIResponse};
use crate::schema::doors::dsl::*;

// https://jsdw.me/posts/rust-asyncawait-preview/

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "doors"]
pub struct NewDoor<'a> {
    #[validate(length(min = "4"))]
    name: &'a str,
    address: &'a str,
    buzzer_url: &'a str,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    status: i32,
    message: String,
}

fn buzz(challenge: String, door_buzzer_url: String) -> Result<String, String> {
    // TODO make it async
    let client = Client::new();
    let code = calculate_hash(&challenge);
    let s = format!("{}/buzz1/{}", door_buzzer_url, code.to_string());
    let url = Url::parse(&s).unwrap();
    let data = json!({ "message": challenge });

    let mut response = match client.post(url).json(&data).send() {
        Ok(x) => x,
        Err(err) => return Err(format!("Error occurred when buzzing: {:?}", err)),
    };

    let res: ResponseData = match response.json() {
        Ok(x) => x,
        Err(err) => return Err(format!("Response data is broken: {:?}", err)),
    };
    debug!("Buzz returned: {}", res.message);

    match res.status {
        400..=500 => return Err(format!("Got error code: {}", res.status)),
        200 => debug!("OK"),
        _ => (),
    };

    Ok("success".to_string())
}

pub fn get_challenge<'a>(door_buzzer_url: &'a str) -> Result<String, String> {
    // TODO make it async
    let client = Client::new();
    let s = format!("{}/challenge", door_buzzer_url);
    let url = Url::parse(&s).expect(&format!("Cannot parse URL: {}", s));

    let mut response = match client.post(url).send() {
        Ok(x) => x,
        Err(err) => return Err(format!("Could not contact host: {:?}", err)),
    };

    let challenge: ResponseData = match response.json() {
        Ok(x) => x,
        Err(err) => return Err(format!("Response data is broken: {:?}", err)),
    };

    match challenge.status {
        400..=500 => return Err(format!("Got error code: {}", challenge.status)),
        200 => debug!("Got challenge, OK"),
        _ => (),
    };

    debug!("Got status {} challenge {}", challenge.status, challenge.message);
    Ok(challenge.message)
}

// example Diesel usage
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8

#[post("/door", data = "<door_data>", format = "application/json")]
pub fn create_door(
    conn: db::Conn,
    _auth: Auth,
    _admin: AdminUser,
    door_data: Json<NewDoor>,
) -> APIResponse {
    let new_door = NewDoor {
        name: door_data.name,
        address: door_data.address,
        buzzer_url: door_data.buzzer_url,
    };

    // TODO: also try `get_result()` here
    // TODO: check all errors
    // https://docs.diesel.rs/diesel/result/enum.DatabaseErrorKind.html
    let insert_result = diesel::insert_into(doors).values(&new_door).execute(&*conn);
    if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =
        insert_result
    {
        debug!("Door with name {} already exist", &new_door.name);
    } else {
        debug!("Door with name {} created", &new_door.name);
    }
    // TODO: remove this panic
    let door: Door = doors
        .filter(name.eq(&new_door.name))
        .first(&*conn)
        .unwrap_or_else(|_| panic!("error getting doors with name={}", new_door.name));
    let resp_data = json!({ "door": door });
    created().data(resp_data)
}

#[get("/doors", format = "application/json")]
pub fn get_doors(conn: db::Conn, _auth: Auth, _admin: AdminUser) -> JsonValue {
    let doors_rs = doors.load::<Door>(&*conn).expect("error retrieving doors");
    json!({ "doors": doors_rs })
}

#[get("/door/<door_id>", format = "application/json")]
pub fn get_door(conn: db::Conn, _auth: Auth, door_id: i32) -> APIResponse {
    let door_res: QueryResult<Door> = doors.find(door_id).first(&*conn);

    match door_res {
        Ok(door_data) => {
            let j = json!({ "door": door_data });
            ok().data(j)
        }
        Err(err) => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("Could not find record for door_id={}: {:?}",
                                  door_id, err)
            });
            bad_request().data(resp_data)
        }
    }
}

#[delete("/door/<door_id>")]
pub fn delete_door(conn: db::Conn, _auth: Auth, _admin: AdminUser, door_id: i32) -> APIResponse {
    let err_msg = format!("Cannot delete door_id={}", door_id);
    diesel::delete(doors.filter(doors::id.eq(door_id))).execute(&*conn).expect(&err_msg);
    no_content()
}

#[post("/door/<door_id>", format = "application/json")]
pub fn buzz_door(conn: db::Conn, _auth: Auth, _user: User, door_id: i32) -> APIResponse {
    let door_res: QueryResult<Door> = doors.find(door_id).first(&*conn);

    match door_res {
        Ok(door_data) => {
            // TODO: make these async
            // TODO: manage errors and return a 40x
            let challenge =
                get_challenge(&door_data.buzzer_url).expect("Failed to get the challenge");

            let buzz_result = buzz(challenge, door_data.buzzer_url).expect("Could not buzz door");
            debug!("Buzz result is: {}", buzz_result);
            // TODO: here better a 204
            let resp_data = json!({
                "status": "OK",
                "detail": format!("Buzzing door {}: {}", door_data.id, buzz_result)
            });
            created().data(resp_data)
        }
        Err(err) => {
            let resp_data = json!({
                "status": "error",
                "detail": format!("Could not find record for door_id={}: {:?}",
                                  door_id, err)
            });
            bad_request().data(resp_data)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Door;
    use crate::routes::doors::NewDoor;
    use crate::schema::doors::dsl::*;
    use diesel::prelude::*;
    use diesel::sqlite::Sqlite;
    use std::env;

    fn get_connection() -> SqliteConnection {
        let database_url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL in env");
        SqliteConnection::establish(&database_url).expect("Could not establish connection")
    }

    fn add_test_door(conn: &SqliteConnection) -> Door {
        let new_door = NewDoor {
            name: "test-door",
            address: "https://buzzer.whatever.de",
            buzzer_url: "http://111.222.333.444",
        };

        let insert_res = diesel::insert_into(doors).values(&new_door).execute(conn);
        match insert_res {
            Ok(_) => {}
            Err(err) => {
                panic!("Insert failed: {:?}", err);
            }
        }

        let door = doors
            .filter(name.eq(&new_door.name))
            .first(&*conn)
            .expect(&format!("error getting doors with name={}", new_door.name));
        door
    }

    fn setup() -> SqliteConnection {
        get_connection()
    }

    fn teardown() {
        let conn = get_connection();
        diesel::delete(doors).execute(&conn).expect("Cannot prune doors table");
    }

    #[test]
    fn test_debug_sql() {
        let conn = setup();
        add_test_door(&conn);

        let q = doors.filter(name.eq("front-door"));
        let sql = diesel::debug_query::<Sqlite, _>(&q).to_string();
        let regexp = "select";
        assert_eq!(
            sql.to_lowercase().contains(regexp),
            true,
            "Debug SQL query returned wrong: {}: got {}",
            regexp,
            sql
        );
        teardown();
    }

    #[test]
    fn test_get() {
        let conn = setup();
        let door_data = add_test_door(&conn);

        let door: Door = doors.find(door_data.id).first(&conn).unwrap();
        assert_eq!(1, door.id);
        teardown();
    }

}
