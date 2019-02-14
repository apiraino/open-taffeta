#![allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::Door;
use crate::schema::doors;
// If a module manages more tables, keep dsl imports in functions
// https://gitter.im/diesel-rs/diesel?at=5b74459749932d4fe4e690b8
use crate::schema::doors::dsl::*;
use crate::responses::{bad_request, ok, no_content, created, APIResponse};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;
use validator_derive::Validate;
use serde_derive::{Serialize, Deserialize};
use crate::auth::Auth;

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "doors"]
pub struct NewDoor {
    #[validate(length(min = "4"))]
    name: String,
    address: String,
}

// example Diesel usage
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8

#[post("/door", data = "<door_data>", format = "application/json")]
pub fn create_door(conn: db::Conn, _auth: Auth, door_data: Json<NewDoor>) -> APIResponse {
    let new_door = NewDoor {
        name: door_data.name.clone(),
        address: door_data.address.clone(),
    };

    // TODO: also try `get_result()` here
    // TODO: check all errors
    // https://docs.diesel.rs/diesel/result/enum.DatabaseErrorKind.html
    let insert_result = diesel::insert_into(doors).values(&new_door).execute(&*conn);
    if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =
        insert_result
    {
        println!(">>> door with name {} already exist", &new_door.name);
    } else {
        println!(">>> door with name {} created", &new_door.name);
    }
    let door: Door = doors
        .filter(name.eq(&new_door.name))
        .first(&*conn)
        .unwrap_or_else(|_| panic!("error getting doors with name={}", new_door.name));
    let resp_data = json!({ "door": door });
    created().data(resp_data)
}

#[get("/doors", format = "application/json")]
pub fn get_doors(conn: db::Conn, _auth: Auth) -> JsonValue {
    let doors_rs = doors.load::<Door>(&*conn).expect("error retrieving doors");
    json!({ "doors": doors_rs })
}

#[get("/door/<door_id>", format = "application/json")]
pub fn get_door(conn: db::Conn, _auth: Auth, door_id: i32) -> APIResponse {
    let door_res : QueryResult<Door> = doors
        .find(door_id)
        .first(&*conn);

    match door_res {
        Ok(door_data) => {
            let j = json!({ "door": door_data });
            ok().data(j)
        },
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

#[delete("/door/<door_id>", format = "application/json")]
pub fn delete_door(conn: db::Conn, _auth: Auth, door_id: i32) -> APIResponse {
    let err_msg = format!("Cannot delete door_id={}", door_id);
    diesel::delete(doors.filter(id.eq(door_id))).execute(&*conn)
        .expect(&err_msg);
    no_content()
}

#[post("/door/<door_id>", format = "application/json")]
pub fn buzz_door(conn: db::Conn, _auth: Auth, door_id: i32) -> APIResponse {
    let door_res : QueryResult<Door> = doors
        .find(door_id)
        .first(&*conn);

    match door_res {
        Ok(door_data) => {
            // TODO: make an async call to the device
            let resp_data = json!({
                "status": "ok",
                "detail": format!("Buzzing door {} on endpoint {}",
                                  door_data.id, door_data.address)
            });
            ok().data(resp_data)
        },
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
    use std::env;
    use diesel::prelude::*;
    use diesel::sqlite::Sqlite;
    use crate::routes::doors::NewDoor;
    use crate::schema::doors::dsl::*;
    use crate::models::Door;

    fn get_connection() -> SqliteConnection {
        let database_url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL in env");
        SqliteConnection::establish(&database_url).expect("Could not establish connection")
    }

    fn add_test_door(conn: &SqliteConnection) -> Door {
        let new_door = NewDoor {
            name: String::from("test-door"),
            address: String::from("https://buzzer.whatever.de")
        };

        let insert_res = diesel::insert_into(doors).values(&new_door).execute(conn);
        match insert_res {
            Ok(_) => {},
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
        diesel::delete(doors).execute(&conn)
            .expect("Cannot prune doors table");
    }

    #[test]
    fn test_debug_sql() {
        let conn = setup();
        add_test_door(&conn);

        let q = doors.filter(name.eq("front-door"));
        let sql = diesel::debug_query::<Sqlite, _>(&q).to_string();
        println!(">>> SQL: {:?}", sql);
        teardown();
    }

    #[test]
    fn test_get() {
        let conn = setup();
        let door_data = add_test_door(&conn);

        let door : Door = doors
            .find(door_data.id)
            .first(&conn)
            .unwrap();
        println!(">>> door {:?}", door);
        teardown();
    }
}
