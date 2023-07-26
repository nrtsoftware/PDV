use crate::{prelude::*, error};

struct User {
  name: String,
  login: String,
  password: String,
  function: String,
}

impl User  {
  fn new(name: String, login: String, password: String, function: String) -> User {
    User { name, login, password, function}
  }
  fn get_name(&self) -> String {
    self.name.clone()
  }
  fn check_function(&self) -> String {
    self.function.clone()
  }
  fn change_password(&mut self, password: String) -> String {
    self.password = password;
    return "Password alterado com sucesso".to_string()
  }
  
}
// trait login{
//   fn 
// }
