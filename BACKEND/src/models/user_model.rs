
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub user_id: String,
  pub user_name: String,
  pub user_email: String,
  pub hashed_password: String,

  pub user_image: String,

  pub verified: bool,
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewUserDTO {
  //pub user_id: String,
  pub user_name: String,
  pub user_email: String,
  pub password: String,

  //pub user_image: String,

  //pub verified: bool,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDTO {
  pub user_id: String,
  pub user_name: String,
  pub user_email: String,
  //pub hashed_password: String,

  pub user_image: String,

  pub verified: bool,
}