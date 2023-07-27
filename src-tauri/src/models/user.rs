use rusqlite::Connection;

use crate::{prelude::*, error};
use crate::models::model::*;


struct UserData {
  name: String,
  password: String,
}
pub struct User {
  table: String,
  model: std::result::Result<Connection, rusqlite::Error>,
  data: UserData,
}

impl User {
  pub fn new() -> User {
    // CONSTS
    const TABLE_NAME: &str = "User";
    
    
    let model: std::result::Result<Connection, rusqlite::Error>  = model_connect();
    let table: String = TABLE_NAME.to_string();
    let data: UserData = UserData {
        name: "".to_string(),
        password: "".to_string(),
    };
    User {table, model, data}
  }

  pub fn set_name_data(&mut self, name: String) {
    self.data.name = name;
  }
  pub fn set_password_data(&mut self, password: String) {
    self.data.password = password;
  }

  pub fn find_by_name(&self) -> Result<Vec<UserData>> {
    match &self.model {
        Ok(connection) => {
            let mut stmt = connection.prepare(&format!("SELECT name, password FROM {} WHERE name = ?", self.table))?;
            let mut rows = stmt.query(&[&self.data.name])?;

            let mut results = Vec::new();
            while let Some(row) = rows.next().map_err(|err| err.to_string())? {
                let name: String = row.get(0)?;
                let password: String = row.get(1).map_err(|err| err.to_string())?;
                let data: UserData = UserData { name, password };
                results.push(data);
            }

            Ok(results)
        },
        Err(err) => {
            Err(err)
        }
    }
}

}

// trait login{
//   fn 
// }
