
use rocket::http::Status;
use rocket::State;

use mongodb::{Database, Cursor};
use mongodb::bson::doc;
use futures::TryStreamExt;

use chrono;
//use either::{self, Either};
use crate::util::my_either::MyEither as Either;

// modules
use crate::models::user_model::User;
use crate::models::typed_psw_record_model::{TypedPswRecord, NewTypedPswRecordDTO};
use crate::models::raw_psw_record_model::{RawPswRecord, NewRawPswRecordDTO};
use crate::models::http_exception::HttpException;
use crate::util::gen_simple_hash::gen_simple_hash;

// helpers
type _AnyNewRecordDTO = Either<NewTypedPswRecordDTO, NewRawPswRecordDTO>;
type _AnyRecord = Either<TypedPswRecord, RawPswRecord>;








pub async fn create_new_record(new_record: _AnyNewRecordDTO, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<_AnyRecord>("pswRecords");


  // get new id
  let mut new_record_id: String = String::new();
  let mut _iters: i32 = 0;
  loop {
    if _iters < 10 {
      new_record_id = "R-".to_owned() + gen_simple_hash(11).as_ref();

    } else if _iters == 10 {
      new_record_id += "-T";

    } else {
      new_record_id += "T";
    }

    let _record_exist: bool = record_collection.find_one(doc! {"owner_id": &user.user_id, "record_id": &new_record_id}).await.unwrap().is_some();
    if _record_exist == false { break; }
    _iters += 1;
  }


  // construct and insert
  match new_record {
    Either::Left(i) => {

      // // construct
      let ready_record = TypedPswRecord {
        owner_id: (&user.user_id).to_string(),
        record_id: (&new_record_id).to_string(),
        app_ico: i.app_ico,
        app_name: i.app_name,
        account_name: i.account_name,
        encoded_login: i.encoded_login,
        encoded_password: i.encoded_password,
        tags: i.tags,
        created_at: chrono::Local::now().to_rfc2822(),
        _record_type: "TYPED".to_string(),
        _id: Default::default(), // for mongodb
      };

      // // insert
      let _ready_record: _AnyRecord = Either::Left(ready_record);
      let _ = record_collection.insert_one(_ready_record).await.expect("Error while inserting");

    },
    Either::Right(j) => {

      // // construct
      let ready_record = RawPswRecord {
        owner_id: (&user.user_id).to_string(),
        record_id: (&new_record_id).to_string(),
        app_ico: j.app_ico,
        app_name: j.app_name,
        raw_content: j.raw_content,
        tags: j.tags,
        created_at: chrono::Local::now().to_rfc2822(),
        _record_type: "RAW".to_string(),
        _id: Default::default(), // for mongodb
      };

      // // insert
      let _ready_record: _AnyRecord = Either::Right(ready_record);
      let _ = record_collection.insert_one(_ready_record).await.expect("Error while inserting");

    }
  };


  Ok(())
}












pub async fn path_record(patched_record: _AnyRecord, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<_AnyRecord>("pswRecords");

  // retrieve some fields... i <3 rust (no)
  let _owner_id = &patched_record.clone().either(
    |i: TypedPswRecord| i.owner_id,
    |j: RawPswRecord| j.owner_id
  );
  let _record_id = &patched_record.clone().either(
    |i: TypedPswRecord| i.record_id,
    |j: RawPswRecord| j.record_id
  );
  let _created_at = &patched_record.clone().either(
    |i: TypedPswRecord| i.created_at,
    |j: RawPswRecord| j.created_at
  );
  let _record_type = &patched_record.clone().either(
    |i: TypedPswRecord| i._record_type,
    |j: RawPswRecord| j._record_type
  );
  let _actual_record_type = &patched_record.clone().either(
    |_: TypedPswRecord| "TYPED".to_string(),
    |_: RawPswRecord| "RAW".to_string()
  );



  // validate patched record
  if _owner_id != &user.user_id {
    return Err( HttpException {
      status: Status::UnprocessableEntity,
      message: "Field 'owner_id' is different from user's id".to_string()
    });
  }
  if chrono::DateTime::parse_from_rfc2822( &_created_at ).is_err() {
    return Err( HttpException {
      status: Status::UnprocessableEntity,
      message: "Field 'created_at' is not rfc2822 time".to_string()
    });
  }
  if _record_type != _actual_record_type {
    return Err( HttpException {
      status: Status::UnprocessableEntity,
      message: format!("Field '_record_type' do not match actual record type ('{}')", _actual_record_type).to_string()
    });
  }


  // check if such id valid
  let _record_exists = record_collection.find_one(
    doc! {"owner_id": &user.user_id, "record_id": _record_id}
  ).await.unwrap().is_some();

  if _record_exists == false {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "There is no record with such id".to_string()
    });
  }


  
  // patch
  let _ = record_collection.find_one_and_replace(
    doc! {"owner_id": &user.user_id, "record_id": _record_id},
    patched_record
  ).await.unwrap();


  Ok(())
}












pub async fn delete_record(target_id: &str, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<_AnyRecord>("pswRecords");


  // check if such id valid
  let _record_exists = record_collection.find_one(
    doc! {"owner_id": &user.user_id, "record_id": target_id}
  ).await.unwrap().is_some();

  if _record_exists == false {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "There is no record with such id".to_string()
    });
  }


  // delete
  let _ = record_collection.find_one_and_delete(
    doc! {"owner_id": &user.user_id, "record_id": target_id}
  ).await.unwrap();


  Ok(())
}










pub async fn get_single_record(target_id: &str, user: User, mongo: &State<Database>) -> Result<_AnyRecord, HttpException> {

  let record_collection = mongo.collection::<_AnyRecord>("pswRecords");

  // get record
  match record_collection.find_one(
    doc! {"owner_id": &user.user_id, "record_id": target_id}
  ).await.unwrap() {

    Some(r) => {
      return Ok(r);
    },

    None => {
      return Err( HttpException {
        status: Status::BadRequest,
        message: "There is no record with such id".to_string()
      });
    }
  }

}









pub async fn get_all_records(user: User, mongo: &State<Database>) -> Result<Vec<_AnyRecord>, HttpException> {

  let record_collection = mongo.collection::<_AnyRecord>("pswRecords");

  // get all records
  let records_cursor: Cursor<_AnyRecord> = record_collection.find(
    doc! {"owner_id": &user.user_id}
  ).await.unwrap();

  let records: Vec<_AnyRecord> = records_cursor.try_collect().await.unwrap();

  Ok(records)
}