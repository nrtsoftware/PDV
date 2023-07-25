// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(unused)] // Comment soon as possible

mod prelude;
mod utils;
mod error;
mod migrations;
mod controllers;
use controllers::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    password: String,
    // data: Option<Vec<u8>>,
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
// }


fn main() {
    // controllers::boot();
    // migrations::boot();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                //user 
                controllers::user_controller::database_test,
                controllers::user_controller::login_user,
                // more controllers
            ]
        )

        .plugin(tauri_plugin_store::Builder::default().build())
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
