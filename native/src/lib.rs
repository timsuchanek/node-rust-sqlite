#[macro_use]
extern crate neon;
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

extern crate rusqlite;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};

use neon::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: i32,
}

fn get_array_with_object(mut cx: FunctionContext) -> JsResult<JsValue> {
    let user = User {
        id: 0,
        name: "Hans not from the database".to_string(),
        email: "a@a.de".to_string(),
        age: 30,
    };

    let users = vec![user];

    let js_value = neon_serde::to_value(&mut cx, &users)?;

    Ok(js_value)
}

fn sqlite_example(mut cx: FunctionContext) -> JsResult<JsValue> {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE user (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL,
                  age             INTEGER NOT NULL
        )",
        NO_PARAMS,
    )
    .unwrap();

    let me = User {
        id: 0,
        name: "Hans from the database".to_string(),
        email: "a@a.de".to_string(),
        age: 30,
    };

    conn.execute(
        "INSERT INTO user (name, email, age)
                  VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.email as &ToSql, &me.age],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, email, age FROM user")
        .unwrap();

    let users_iter = stmt
        .query_map(NO_PARAMS, |row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            age: row.get(3),
        })
        .unwrap();

    let users_vec = users_iter.map(|user| user.unwrap()).collect::<Vec<User>>();
    let js_value = neon_serde::to_value(&mut cx, &users_vec)?;

    Ok(js_value)
}

register_module!(mut m, {
    m.export_function("getUsers", get_array_with_object)?;
    m.export_function("getUsersSqlite", sqlite_example)
});
