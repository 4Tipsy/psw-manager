
use rocket::{get, post, State};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::time::Duration;
use rocket::serde::json::{json, Json, to_value as to_json_value};
use rocket::fs::{TempFile, NamedFile};
use rocket::form::Form;

use serde::Deserialize;
use mongodb::Database;

use std::path::PathBuf;

// modules
use crate::services::UserService;
use crate::guards::AuthGuard::AuthGuard;
use crate::models::User::{NewUserDTO, UserDTO};
use crate::models::HttpException::HttpException;
use crate::util::api_responses::{ApiJsonResponse, ApiTextResponse};
use crate::ConfigModel;
use crate::ACCESS_TOKEN_LIVES;










#[post("/user-serv/create-new-user", data = "<req>")]
pub async fn create_new_user(req: Json<NewUserDTO>, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<(), HttpException> = UserService::create_new_user(
    req.into_inner(),
    mongo,
    config
  ).await;


  match r {
    Ok(_) => {
      Ok( ApiTextResponse {
        value: "ok".to_string(),
        status: Status::Ok
      })
    },
    Err(e) => {
      Err( ApiJsonResponse {
        value: json!({"err": e.message}),
        status: e.status
      })
    },
  }

}






#[get("/user-serv/get-current-user")]
pub async fn get_current_user(auth_guard: AuthGuard) -> Result<ApiJsonResponse, ApiJsonResponse> {

  let r: Result<UserDTO, HttpException> = UserService::get_current_user(auth_guard.value).await;

  match r {
    Ok(u) => {
      Ok( ApiJsonResponse {
        value: to_json_value(u).unwrap(),
        status: Status::Ok
      })
    },
    Err(e) => {
      Err( ApiJsonResponse {
        value: json!({"err": e.message}),
        status: e.status
      })
    },
  }

}








#[derive(Deserialize)]
pub struct _LoginReq {
  pub user_email: String,
  pub user_password: String,
}

#[post("/user-serv/login", data = "<req>")]
pub async fn login(req: Json<_LoginReq>, jar: &CookieJar<'_>, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<String, HttpException> = UserService::get_user_token(
    &req.user_email,
    &req.user_password,
    mongo,
    config
  ).await;

  match r {
    Ok(access_token) => {

      let access_cookie = Cookie
        ::build(("psw-manager.access_token", access_token))
        .max_age(Duration::days(ACCESS_TOKEN_LIVES));

      jar.add(access_cookie);


      Ok( ApiTextResponse {
        value: "ok".to_string(),
        status: Status::Ok
      })
    },
    Err(e) => {
      Err( ApiJsonResponse {
        value: json!({"err": e.message}),
        status: e.status
      })
    },
  }


}






#[get("/user-serv/get-user-image")]
pub async fn get_user_image(auth_guard: AuthGuard, config: &State<ConfigModel>) -> Result<NamedFile, ApiJsonResponse> {

  let r: Result<PathBuf, HttpException> = UserService::get_user_image(
    auth_guard.value,
    config
  ).await;

  match r {
    Ok(p) => {
      Ok(NamedFile::open(p).await.unwrap())
    },
    Err(e) => {
      Err( ApiJsonResponse {
        value: json!({"err": e.message}),
        status: e.status
      })
    },
  }


}





#[post("/user-serv/update-user-image", data = "<file>")]
pub async fn update_user_image(auth_guard: AuthGuard, file: Form<TempFile<'_>>, config: &State<ConfigModel>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<(), HttpException> = UserService::update_user_image(
    auth_guard.value,
    file,
    config
  ).await;

  match r {
    Ok(_) => {
      Ok( ApiTextResponse {
        value: "ok".to_string(),
        status: Status::Ok
      })
    },
    Err(e) => {
      Err( ApiJsonResponse {
        value: json!({"err": e.message}),
        status: e.status
      })
    },
  }


}