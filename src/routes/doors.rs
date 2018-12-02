#[allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::Door;
use crate::schema::doors;
use crate::schema::doors::dsl::*;
use crate::responses::{bad_request, created, unauthorized, APIResponse};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use rocket_contrib::{Json, Value};
use validator::Validate;
use auth::Auth;

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "doors"]
pub struct NewDoor {
    #[validate(length(min = "4"))]
    name: String,
}

// example Diesel usage
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8

#[post("/door", data = "<door_data>", format = "application/json")]
fn create_door(conn: db::Conn, auth: Auth, door_data: Json<NewDoor>) -> APIResponse {
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

#[cfg(test)]
mod tests {
    use crate::models::Door;
    use diesel::prelude::*;
    use diesel::sqlite::Sqlite;
    use std::env;

    pub fn get_connection() -> SqliteConnection {
        let database_url = env::var("DATABASE_URL").unwrap();
        SqliteConnection::establish(&database_url).unwrap()
    }

    fn setup() -> SqliteConnection {
        let conn = get_connection();
        conn
    }

    #[test]
    fn test_debug_sql() {
        use crate::schema::doors::dsl::*;

        let q = doors.filter(name.eq("front-door"));
        let sql = diesel::debug_query::<Sqlite, _>(&q).to_string();
        println!(">>> SQL: {:?}", sql);
    }

    #[test]
    fn test_get_records() {
        use crate::schema::doors::dsl::*;

        let conn = setup();
        let doors_existing: Vec<Door> = doors
            .filter(name.eq("front-door"))
            .load(&conn)
            .expect(&format!("boom"));
        assert_eq!(1, doors_existing.len());
    }

    #[test]
    fn test_count_records() {
        use crate::schema::doors::dsl::*;

        let conn = setup();
        // select count: <diesel::sql_types::BigInt> -> returns an i64
        let door_count: i64 = doors
            .filter(name.eq("front-door"))
            .count()
            .get_result(&conn)
            .expect("boom");
        assert_eq!(1, door_count);
    }

}
