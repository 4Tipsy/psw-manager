
use rocket::http::Status;
use rocket::State;

use mongodb::{Database, Cursor};
use mongodb::bson::doc;
use futures::TryStreamExt;

use chrono;

// modules
use crate::models::user_model::User;
use crate::models::psw_record_model::{PswRecord, NewPswRecordDTO};
use crate::models::http_exception::HttpException;
use crate::util::gen_simple_hash::gen_simple_hash;







pub async fn create_new_record(new_record: NewPswRecordDTO, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<PswRecord>("pswRecords");


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

    let _record_exist: bool = record_collection.find_one(doc! {"owner_id": &new_record_id}).await.unwrap().is_some();
    if _record_exist == false { break; }
    _iters += 1;
  }


  // insert new record
  let ready_record = PswRecord {
    owner_id: user.user_id,
    record_id: new_record_id,
    app_ico: new_record.app_ico,
    app_name: new_record.app_name,
    account_name: new_record.account_name,
    encoded_login: new_record.encoded_login,
    encoded_password: new_record.encoded_password,
    tags: new_record.tags,
    created_at: chrono::Local::now().to_rfc2822(),
  };
  let _ = record_collection.insert_one(ready_record).await.expect("Error while inserting");

  Ok(())
}












pub async fn path_record(patched_record: PswRecord, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<PswRecord>("pswRecords");


  // validate patched record
  if &patched_record.owner_id != &user.user_id {
    return Err( HttpException {
      status: Status::UnprocessableEntity,
      message: "Field 'owner_id' is different from user's id".to_string()
    });
  }
  if chrono::DateTime::parse_from_rfc2822( &patched_record.created_at ).is_err() {
    return Err( HttpException {
      status: Status::UnprocessableEntity,
      message: "Field 'created_at' is not rfc2822 time".to_string()
    });
  }


  // check if such id valid
  let _record_exists = record_collection.find_one(
    doc! {"owner_id": &user.user_id, "record_id": &patched_record.record_id}
  ).await.unwrap().is_some();

  if _record_exists == false {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "There is no record with such id".to_string()
    });
  }


  // patch
  let _ = record_collection.find_one_and_replace(
    doc! {"owner_id": &user.user_id, "record_id": &patched_record.record_id},
    patched_record
  ).await.unwrap();


  Ok(())
}












pub async fn delete_record(target_id: &str, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let record_collection = mongo.collection::<PswRecord>("pswRecords");


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










pub async fn get_single_record(target_id: &str, user: User, mongo: &State<Database>) -> Result<PswRecord, HttpException> {

  let record_collection = mongo.collection::<PswRecord>("pswRecords");

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









pub async fn get_all_records(user: User, mongo: &State<Database>) -> Result<Vec<PswRecord>, HttpException> {

  let record_collection = mongo.collection::<PswRecord>("pswRecords");

  // get all records
  let records_cursor: Cursor<PswRecord> = record_collection.find(
    doc! {"owner_id": &user.user_id}
  ).await.unwrap();

  let records: Vec<PswRecord> = records_cursor.try_collect().await.unwrap();

  Ok(records)
}