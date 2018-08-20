#[allow(proc_macro_derive_resolution_fallback)]
use crate::db;
use crate::models::Door;
use crate::schema::doors;
use crate::schema::doors::dsl::*;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use rocket_contrib::{Json, Value};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, Insertable)]
#[table_name = "doors"]
pub struct NewDoor {
    #[validate(length(min = "4"))]
    name: String,
}

// example Diesel usage
// https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8

#[post("/door", data = "<door_data>", format = "application/json")]
fn create_door(conn: db::Conn, door_data: Json<NewDoor>) -> Json<Value> {
    // Keep dsl imports in functions
    // https://gitter.im/diesel-rs/diesel?at=5b74459749932d4fe4e690b8
    // use crate::schema::doors::dsl::*;
    let new_door = NewDoor {
        name: door_data.name.clone(),
    };

    // TODO: also try `get_result()` her
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
    Json(json!({ "door": door }))
}

#[cfg(test)]
mod tests {
    use crate::models::Door;
    use diesel::prelude::*;
    use diesel::sqlite::Sqlite;

    // setup() and teardown() are arbitrary names
    // see: Rust Book - "Test Organization"
    pub fn get_connection() -> SqliteConnection {
        let database_url = dotenv!("DATABASE_URL");
        SqliteConnection::establish(&database_url).unwrap()
    }

    fn setup() -> SqliteConnection {
        let conn = get_connection();
        conn
    }

    fn teardown() {}

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
        teardown();
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
        teardown();
    }

}
