
use serde::Deserialize;
use serde_yaml;

use std::fs;
use std::panic;






#[derive(Debug, Deserialize)]
pub struct ConfigModel {
  pub port: i32,
  pub address: String,
  pub storage_path: String,
  pub psw_secret: String,
  pub db_mongo_uri: String,
  pub jwt_secret: String,
  pub jwt_epoch: i32,
  pub cors_allowed_hosts: Vec<String>,

  pub client_static_path: String,
}








pub fn load_config() -> ConfigModel {
  let _config: String = fs::read_to_string("./Config.yaml").expect("Can not find or read 'Config.yaml'");
  let config: ConfigModel = serde_yaml::from_str(&_config).expect("Invalid 'Config.yaml'");


  // check
  match fs::exists(&config.storage_path) {
    Ok(r) => {
      if !r { panic!("Field 'storage_path' should refer existing path") }
    }
    Err(_) => { panic!("Field 'storage_path' should refer existing path") }
  }


  // check
  match fs::exists(&config.client_static_path) {
    Ok(r) => {
      if !r { panic!("Field 'client_static_path' should refer existing path") }
    }
    Err(_) => { panic!("Field 'client_static_path' should refer existing path") }
  }



  return config;
}