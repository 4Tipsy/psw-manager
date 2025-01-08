
// _
mod models;
mod routes;
mod services;
mod guards;
mod util;


use rocket::{launch, catch, routes, catchers};
use rocket::figment::Figment;
use rocket::http::Status;
use rocket::serde::json::json;
use rocket::Config as RocketConfig;

use serde::Deserialize;
use serde_yaml;
use mongodb::{Client, Database};

use std::fs;


// modules
use crate::routes::UserRoutes;
use crate::routes::PswRecordRoutes;
use crate::routes::StaticRoutes;
use crate::guards::CorsFairing;
use crate::util::api_responses::ApiJsonResponse;




static ACCESS_TOKEN_LIVES: i64 = 24*30;
//static REFRESH_TOKEN_LIVES: i64 = 24*30;




// config
#[derive(Debug, Deserialize)]
pub struct ConfigModel {
  pub port: i32,
  pub address: String,
  pub storage_path: String,
  pub psw_secret: String,
  pub db_mongo_uri: String,
  pub db_redis_uri: String,
  pub jwt_secret: String,
  pub jwt_epoch: i32,
  pub cors_allowed_hosts: Vec<String>,
}







// some catchers
#[catch(404)]
async fn handle_404() -> Status {
  Status::NotFound
}
#[catch(500)]
async fn handle_500() -> ApiJsonResponse {
  ApiJsonResponse {
    status: Status::InternalServerError,
    value: json!({"err": "Internal server error".to_string()})
  }
}
/* shouldn't be like that */
#[catch(401)]
async fn handle_401__AuthGuard() -> ApiJsonResponse {
  ApiJsonResponse {
    status: Status::Unauthorized,
    value: json!({"err": "Unauthorized".to_string()})
  }
}








#[launch]
async fn launch() -> _ {

  // load config
  let _config: String = fs::read_to_string("./Config.yaml").expect("Can not find or read 'Config.yaml'");
  let config: ConfigModel = serde_yaml::from_str(&_config).expect("Invalid 'Config.yaml'");

  // load database
  let mongo: Database = Client::with_uri_str( &config.db_mongo_uri )
    .await
    .expect("Unable to connect to MongoDB")
    .database("PswManager");


  // start rocket
  rocket::build()
    .configure( RocketConfig::figment().merge(
      Figment::new()
      .join(("port", &config.port)) // set port
      .join(("address", &config.address)) // set address
    ))
    .mount("/", routes![
      UserRoutes::create_new_user, UserRoutes::get_current_user, UserRoutes::login, UserRoutes::get_user_image, UserRoutes::update_user_image,
      PswRecordRoutes::create_new_record, PswRecordRoutes::patch_record, PswRecordRoutes::delete_record, PswRecordRoutes::get_all_records, PswRecordRoutes::get_single_record,
      StaticRoutes::send_docs_swagger, StaticRoutes::send_docs_redoc, StaticRoutes::send_openapi
    ])
    .register("/", catchers![
      handle_404, handle_500, handle_401__AuthGuard
    ])
    .manage(config)
    .manage(mongo)
    .attach(CorsFairing::CORS) // set cors
}