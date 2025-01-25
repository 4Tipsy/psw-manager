
// _
mod config;
mod models;
mod routes;
mod services;
mod guards;
mod util;


use rocket::{launch, catch, routes, catchers};
use rocket::options;
use rocket::figment::Figment;
use rocket::http::Status;
use rocket::serde::json::json;
use rocket::Config as RocketConfig;

use rocket::fs::{FileServer, Options as FileServerOptions};

use mongodb::{Client, Database};


// modules
use crate::routes::user_routes;
use crate::routes::psw_record_routes;
use crate::routes::static_routes;
use crate::guards::cors_fairing::CorsFairing;
use crate::util::api_responses::{ApiJsonResponse, ApiTextResponse};
use crate::config::{ConfigModel, load_config};




static ACCESS_TOKEN_LIVES: i64 = 24*30;
//static REFRESH_TOKEN_LIVES: i64 = 24*30;






#[options("/<_..>")]
async fn handle_options() {
  /* for some reason, without that fn, cors fails on OPTIONS request */
}




// some catchers
#[catch(404)]
async fn handle_404() -> ApiTextResponse {
  ApiTextResponse {
    status: Status::NotFound,
    value: "Not found".to_string()
  }
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
#[allow(non_snake_case)]
async fn handle_401__auth_guard__() -> ApiJsonResponse {
  ApiJsonResponse {
    status: Status::Unauthorized,
    value: json!({"err": "Unauthorized".to_string()})
  }
}








#[launch]
async fn launch() -> _ {

  // load config
  let config: ConfigModel = load_config();

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
    .mount("/",
      FileServer::new(&config.client_static_path, FileServerOptions::Index) // client static
    )
    .mount("/__api__/", routes![
      user_routes::create_new_user, user_routes::get_current_user, user_routes::login, user_routes::get_user_image, user_routes::update_user_image,
      psw_record_routes::create_new_record, psw_record_routes::patch_record, psw_record_routes::delete_record, psw_record_routes::get_all_records, psw_record_routes::get_single_record,
      static_routes::send_docs_swagger, static_routes::send_docs_redoc, static_routes::send_openapi,
      handle_options
    ])
    .register("/", catchers![
      handle_404, handle_500, handle_401__auth_guard__
    ])
    .manage(config)
    .manage(mongo)
    .attach(CorsFairing) // set cors
}