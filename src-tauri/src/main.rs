// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


use rusqlite::{Connection, Result};



#[tauri::command]
async fn database_test() -> Result<String, String> {
    // Err("This failed!");
    let conn: Connection = Connection::open_in_memory().map_err(|err| err.to_string())?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    ).map_err(|err| err.to_string())?;

    //Some other queries and operations if needed
    let me = Person {
        id: 0,
        name: "Nivaldinho".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    ).map_err(|err| err.to_string())?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").map_err(|err| err.to_string())?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }).map_err(|err| err.to_string())?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok("This worked!".to_string())
}

// #[tauri::command]
// fn database_test() -> Result<(), String> {
//     let conn = Connection::open_in_memory()?;

//     conn.execute(
//         "CREATE TABLE person (
//             id   INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             data BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }






// fn database_testando() -> String {
    // let conn = Connection::open_in_memory()?;
    // conn.execute("
    //     CREATE TABLE person (id INTEGER PRIMARY KEY,
    //     name TEXT NOT NULL,
    //     data BLOB
    //     )",
    //     (), //empty list
    // )?;
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };

    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1 ,?2)",
    //     (&me.name, &me.data),
    // )?;

    // let mut stmt = conn.prepare("SELECT id, name, data FROMR person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(
    //         Person {
    //             id: row.get(0)?,
    //             name: row.get(1)?,
    //             data: row.get(2) ?,
    //         }
    //     )
    // })?;
    // format!("Found person");
    // for person in person_iter {
    //     // println!("Found person {:?}", );
    // }
    // Ok(())
//     format!("Hello! You've been greeted from Rust!");
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![database_test])
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
