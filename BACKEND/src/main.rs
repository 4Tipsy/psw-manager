
// _
mod config;
mod models;
mod routes;
mod services;
mod guards_fairings;
mod util;


use rocket::{launch, catch, routes, catchers};
use rocket::{options, get};
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
use crate::guards_fairings::cors_fairing::CorsFairing;
use crate::guards_fairings::some_headers_fairing::SomeHeadersFairing;
use crate::util::api_responses::{ApiJsonResponse, ApiTextResponse};
use crate::util::rate_limiter::RateLimiter;
use crate::config::{ConfigModel, load_config};




static ACCESS_TOKEN_LIVES: i64 = 24*30; // 30 days
static API_VERSION: &str = "2.2.0";






#[options("/<_..>")]
async fn help_handle_options() {
  /* for some reason, without that fn, cors fails on OPTIONS request */
}

#[get("/ping")]
async fn handle_ping() -> ApiTextResponse {
  /* ping-api request */
  ApiTextResponse {
    status: Status::NonAuthoritativeInformation,
    value: "pong".to_string()
  }
}








// some catchers
#[catch(404)]
async fn handle_404() -> ApiTextResponse {
  ApiTextResponse {
    status: Status::NotFound,
    value: "Not found".to_string()
  }
}
#[catch(422)]
async fn handle_422() -> ApiJsonResponse {
  ApiJsonResponse {
    status: Status::UnprocessableEntity,
    value: json!({"err": "Unprocessable entity. The request was well-formed but was unable to be followed due to semantic errors".to_string()})
  }
}
#[catch(500)]
async fn handle_500() -> ApiJsonResponse {
  ApiJsonResponse {
    status: Status::InternalServerError,
    value: json!({"err": "Internal server error".to_string()})
  }
}
/* shouldn't be like that, but returning from guard is not working for some reason, sadly... */
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
      .join(("ip_header", false)) // disable ip header
    ))
    .mount("/",
      FileServer::new(&config.client_static_path, FileServerOptions::Index) // client static
    )
    .mount("/__api__/", routes![
      user_routes::create_new_user, user_routes::get_current_user, user_routes::login, user_routes::get_user_image, user_routes::update_user_image,
      psw_record_routes::create_new_record, psw_record_routes::patch_record, psw_record_routes::delete_record, psw_record_routes::get_all_records, psw_record_routes::get_single_record,
      static_routes::send_docs_swagger, static_routes::send_docs_redoc, static_routes::send_openapi,
      help_handle_options, handle_ping
    ])
    .register("/", catchers![
      handle_404, handle_422, handle_500, handle_401__auth_guard__
    ])
    .manage(RateLimiter::init(&config._login_rate_limit))
    .manage(config)
    .manage(mongo)
    .attach(CorsFairing) // set cors
    .attach(SomeHeadersFairing)
}
