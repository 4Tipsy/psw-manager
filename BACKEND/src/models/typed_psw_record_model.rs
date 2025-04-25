
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;



#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TypedPswRecord {
  pub owner_id: String,

  pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub account_name: String,
  pub encoded_login: String,
  pub encoded_password: String,

  pub tags: Vec<String>,

  pub created_at: String,


  pub _record_type: String, // "TYPED"


  #[serde(skip_serializing, default)]
  pub _id: Option<ObjectId>, // for mongodb
}




#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct NewTypedPswRecordDTO {
  //pub owner_id: String,

  //pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub account_name: String,
  pub encoded_login: String,
  pub encoded_password: String,

  pub tags: Vec<String>,

  //pub created_at: String,


  //pub _record_type: String, // "TYPED"
}