use crate::{prelude::*, error};

struct User {
  id: u8,
  name: String,
  login: String,
  password: String,
  function: String,
}

impl User  {
  fn new(id: u8, name: String, login: String, password: String, function: String) -> User {
    User {id, name, login, password, function}
  }
  fn get_name(&self) -> String {
    self.name.clone()
  }
  fn check_function(&self) -> String {
    self.function.clone()
  }
  pub fn change_password(&mut self, password: String) -> String {
    self.password = password;
    return "Password alterado com sucesso".to_string()
  }
  
}
// trait login{
//   fn 
// }
