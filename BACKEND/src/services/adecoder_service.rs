
use rocket::http::Status;
use rocket::State;

use mongodb::Database;
use mongodb::bson::doc;

// modules
use crate::models::user_model::User;
use crate::models::adecoder_check_pair::AdecoderCheckPair;
use crate::models::http_exception::HttpException;








pub async fn get_check_pair(user: User, mongo: &State<Database>) -> Result<AdecoderCheckPair, HttpException> {

  let adecoder_check_pairs_collections = mongo.collection::<AdecoderCheckPair>("adecoderCheckPairs");


  match adecoder_check_pairs_collections.find_one(
    doc! {"owner_id": &user.user_id}
  ).await.unwrap() {

    Some(p) => {
      return Ok(p);
    },

    None => {
      return Err( HttpException {
        status: Status::BadRequest,
        message: "There is no record with such id".to_string()
      });
    }
  }

}




pub async fn set_check_pair(encoded_phrase: String, should_be: String, user: User, mongo: &State<Database>) -> Result<(), HttpException> {

  let adecoder_check_pairs_collections = mongo.collection::<AdecoderCheckPair>("adecoderCheckPairs");


  // delete old record
  let _ = &adecoder_check_pairs_collections.find_one_and_delete(doc! {"owner_id": &user.user_id}).await.unwrap();

  // insert one
  let new_pair = AdecoderCheckPair {
    owner_id: user.user_id,
    encoded_phrase: encoded_phrase,
    should_be: should_be,
  };
  let _ = &adecoder_check_pairs_collections.insert_one(new_pair).await.unwrap();


  Ok(())
}