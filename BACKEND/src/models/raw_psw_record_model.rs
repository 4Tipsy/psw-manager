
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;



#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RawPswRecord {
  pub owner_id: String,

  pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub raw_content: String,

  pub tags: Vec<String>,

  pub created_at: String,



  pub _record_type: String, // "RAW"


  #[serde(skip_serializing, default)]
  pub _id: Option<ObjectId>, // for mongodb
}




#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct NewRawPswRecordDTO {
  //pub owner_id: String,

  //pub record_id: String,

  pub app_ico: Option<String>,
  pub app_name: String,

  pub raw_content: String,

  pub tags: Vec<String>,

  //pub created_at: String,


  //pub _record_type: String, // "RAW"
}