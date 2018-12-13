#[allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::Door;
use crate::schema::doors;
use crate::schema::doors::dsl::*;
use crate::responses::{created, APIResponse};
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
}

// example Diesel usage
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8

#[post("/door", data = "<door_data>", format = "application/json")]
pub fn create_door(conn: db::Conn, auth: Auth, door_data: Json<NewDoor>) -> APIResponse {
    // Keep dsl imports in functions
    // https://gitter.im/diesel-rs/diesel?at=5b74459749932d4fe4e690b8
    // use crate::schema::doors::dsl::*;

    let new_door = NewDoor {
        name: door_data.name.clone(),
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
        .expect(&format!("error getting doors with name={}", new_door.name));
    let resp_data = json!({ "door": door });
    created().data(resp_data)
}

#[get("/doors", format = "application/json")]
pub fn get_doors(conn: db::Conn, auth: Auth) -> JsonValue {
    let doors_rs = doors.load::<Door>(&*conn).expect("error retrieving doors");
    json!({ "doors": doors_rs })
}

#[get("/door/<door_id>", format = "application/json")]
pub fn get_door(conn: db::Conn, auth: Auth, door_id: i32) -> JsonValue {
    let door: Vec<Door> = doors
        .filter(id.eq(door_id))
        .load(&*conn)
        .expect(&format!("error retrieving door id={}", door_id));
    json!({ "door": door })
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
        let database_url = env::var("DATABASE_URL").unwrap();
        SqliteConnection::establish(&database_url).unwrap()
    }

    fn add_test_door(conn: &SqliteConnection) {
        let new_door = NewDoor {
            name: String::from("test-door")
        };
        diesel::insert_into(doors).values(&new_door).execute(conn);
    }

    fn setup() -> SqliteConnection {
        get_connection()
    }

    #[test]
    fn test_debug_sql() {
        let q = doors.filter(name.eq("front-door"));
        let sql = diesel::debug_query::<Sqlite, _>(&q).to_string();
        println!(">>> SQL: {:?}", sql);
    }
}
