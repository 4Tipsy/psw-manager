
// _
mod config;
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

use rocket::fs::{FileServer, Options as FileServerOptions};

use mongodb::{Client, Database};


// modules
use crate::routes::UserRoutes;
use crate::routes::PswRecordRoutes;
use crate::routes::StaticRoutes;
use crate::guards::CorsFairing;
use crate::util::api_responses::{ApiJsonResponse, ApiTextResponse};
use crate::config::{ConfigModel, load_config};




static ACCESS_TOKEN_LIVES: i64 = 24*30;
//static REFRESH_TOKEN_LIVES: i64 = 24*30;







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
async fn handle_401__AuthGuard() -> ApiJsonResponse {
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
    .mount("/api/", routes![
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