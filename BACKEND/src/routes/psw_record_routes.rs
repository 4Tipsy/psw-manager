
use rocket::{get, post, patch, delete, State};
use rocket::http::Status;
use rocket::serde::json::{json, Json, to_value as to_json_value};

use serde::Deserialize;
use mongodb::Database;
//use either::Either;
use crate::util::my_either::MyEither as Either;

// modules
use crate::services::psw_record_service;
use crate::guards_fairings::auth_guard::AuthGuard;
use crate::models::typed_psw_record_model::{TypedPswRecord, NewTypedPswRecordDTO};
use crate::models::raw_psw_record_model::{RawPswRecord, NewRawPswRecordDTO};
use crate::models::http_exception::HttpException;
use crate::util::api_responses::{ApiJsonResponse, ApiTextResponse};

// helpers
type _AnyNewRecordDTO = Either<NewTypedPswRecordDTO, NewRawPswRecordDTO>;
type _AnyRecord = Either<TypedPswRecord, RawPswRecord>;








#[post("/record-serv/create-new-record", data="<req>")]
pub async fn create_new_record(req: Json<_AnyNewRecordDTO>, auth: AuthGuard, mongo: &State<Database>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<(), HttpException> = psw_record_service::create_new_record(req.into_inner(), auth.value, mongo).await;

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






#[patch("/record-serv/patch-record", data="<req>")]
pub async fn patch_record(req: Json<_AnyRecord>, auth: AuthGuard, mongo: &State<Database>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<(), HttpException> = psw_record_service::path_record(req.into_inner(), auth.value, mongo).await;

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








#[derive(Debug, Deserialize)]
pub struct _DeleteRecordReq {
  pub target_id: String,
}

#[delete("/record-serv/delete-record", data="<req>")]
pub async fn delete_record(req: Json<_DeleteRecordReq>, auth: AuthGuard, mongo: &State<Database>) -> Result<ApiTextResponse, ApiJsonResponse> {

  let r: Result<(), HttpException> = psw_record_service::delete_record(&req.target_id, auth.value, mongo).await;

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






#[get("/record-serv/get-records")]
pub async fn get_all_records(auth: AuthGuard, mongo: &State<Database>) -> Result<ApiJsonResponse, ApiJsonResponse> {

  let r: Result< Vec<_AnyRecord>, HttpException> = psw_record_service::get_all_records(auth.value, mongo).await;

  match r {
    Ok(j) => {
      Ok( ApiJsonResponse {
        value: to_json_value(j).unwrap(),
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






#[get("/record-serv/get-records/<target_id>")]
pub async fn get_single_record(target_id: &str, auth: AuthGuard, mongo: &State<Database>) -> Result<ApiJsonResponse, ApiJsonResponse> {

  let r: Result<_AnyRecord, HttpException> = psw_record_service::get_single_record(target_id, auth.value, mongo).await;

  match r {
    Ok(j) => {
      Ok( ApiJsonResponse {
        value: to_json_value(j).unwrap(),
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