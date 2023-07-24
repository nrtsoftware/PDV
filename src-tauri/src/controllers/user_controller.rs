use rusqlite::{Connection, Result};
use serde_json::json;
use crate::*;

#[tauri::command]
pub async fn database_test() -> Result<String, String> {
    // Err("This failed!");
    let conn: Connection = Connection::open("./app.sqlite3").map_err(|err| err.to_string())?;
    // Imprimir a string JSON resultante
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS person (
    //         id   INTEGER PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         password TEXT NOT NULL,
    //     )",
    //     (),
    // ).map_err(|err| err.to_string())?;

    //Some other queries and operations if needed
    // let me = Person {
    //     id: 0,
    //     name: "Nivaldinho".to_string(),
    //     data: None,
    // };

    // let me2 = Person {
    //     id: 0,
    //     name: "barosso".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // ).map_err(|err| err.to_string())?;

    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me2.name, &me2.data),
    // ).map_err(|err| err.to_string())?;

    let mut stmt = conn.prepare("SELECT id, name, password FROM person").map_err(|err| err.to_string())?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
        })
    }).map_err(|err| err.to_string())?;
    
    // Criar um vetor de objetos JSON para armazenar os registros da tabela
    let mut data: Vec<serde_json::Value> = Vec::new();

    for person in person_iter {
        let person = person.map_err(|err| err.to_string())?;
        let person_json = json!({
            "id": person.id,
            "name": person.name,
            "password": person.password
        });
        data.push(person_json);
    }

    // Serializar o vetor em uma string JSON
    let json_string = serde_json::to_string(&data).map_err(|err| err.to_string())?;

    // Retornar a string JSON resultante
    Ok(json_string)

}


pub fn search_user() {

}