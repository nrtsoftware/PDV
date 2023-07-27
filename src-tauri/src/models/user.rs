use rusqlite::Connection;

use crate::{prelude::*, error};
use crate::models::model::*;
struct User {
  table: String,
  model: std::result::Result<Connection, rusqlite::Error> ,
}

impl User {
  fn new() -> User {
    let model: std::result::Result<Connection, rusqlite::Error>  = model_connect();
    let table: String = "user".to_string();
    User {table, model}
  }
  fn find_by_name(&self) {
    match &self.model {
        Ok(connection) => {
            connection.prepare(&format!("SELECT name FROM {}", self.table)).unwrap();
        },
        Err(err) => {
            eprintln!("Error connecting to the database: {:?}", err);
        }
    }
}

  
}

fn main() {

}
// trait login{
//   fn 
// }
