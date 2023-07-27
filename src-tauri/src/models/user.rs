use crate::{prelude::*, error};
use crate::models::model::*;
struct User {
  table: String,
  model: Model,
}

impl User {
  fn new() -> User {
    let model: Model = model_connect();
    let table: String = "user".to_string();
    User {table, model}
  }
  fn find_by_name(&self) {
    self.model.select(format!("SELECT name FROM {}", self.table.clone()));
  }
  
}

fn main() {

}
// trait login{
//   fn 
// }
