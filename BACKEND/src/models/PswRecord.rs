
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PswRecord {
  pub owner_id: String,

  pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub account_name: String,
  pub encoded_login: String,
  pub encoded_password: String,

  pub tags: Vec<String>,

  pub created_at: String,
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewPswRecordDTO {
  //pub owner_id: String,

  //pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub account_name: String,
  pub encoded_login: String,
  pub encoded_password: String,

  pub tags: Vec<String>,

  //pub created_at: String,
}