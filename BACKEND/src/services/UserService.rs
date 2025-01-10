
use rocket::State;
use rocket::http::{Status, ContentType};
use rocket::fs::TempFile;
use rocket::form::Form;

use mongodb::Database;
use mongodb::bson::doc;

use jsonwebtoken::{self, Header as JwtHeader, EncodingKey};
use chrono;

use std::path::{Path, PathBuf};
use std::fs;

// modules
use crate::models::User::{User, NewUserDTO, UserDTO};
use crate::models::Tokens::{AccessToken};
use crate::models::HttpException::HttpException;
use crate::util::gen_simple_hash::gen_simple_hash;
use crate::util::password_hasher::{hash_psw, verify_psw};
use crate::config::ConfigModel;
use crate::ACCESS_TOKEN_LIVES;







pub async fn create_new_user(new_user: NewUserDTO, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<(), HttpException> {

  let user_collection = mongo.collection::<User>("users");


  // check if email is taken
  let _email_taken = user_collection.find_one(doc! {"user_email": &new_user.user_email}).await.unwrap().is_some();
  if _email_taken {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "Email is taken".to_string(),
    })
  }


  // get new id
  let mut new_user_id: String = String::new();
  let mut _iters: i32 = 0;
  loop {
    if _iters < 10 {
      new_user_id = "U-".to_owned() + gen_simple_hash(8).as_ref();

    } else if _iters == 10 {
      new_user_id += "-T";

    } else {
      new_user_id += "T";
    }

    let _user_exist: bool = user_collection.find_one(doc! {"user_id": &new_user_id}).await.unwrap().is_some();
    if _user_exist == false { break; }
    _iters += 1;
  }


  // create user image in fs
  let _img_file_name = "avatar_".to_owned() + &new_user_id;
  let user_img_path = Path::new( &config.storage_path ).join( _img_file_name );
  let _ = fs::copy("./data/default_user_image.jpg", user_img_path);


  // insert new user
  let ready_user = User {
    user_id: new_user_id,
    user_name: new_user.user_name,
    user_email: new_user.user_email,
    hashed_password: hash_psw(&new_user.password, &config.psw_secret),
    verified: false,
  };
  let _ = user_collection.insert_one(ready_user).await.expect("Error while inserting");


  Ok(())
}













pub async fn get_user_token(user_email: &str, user_password: &str, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<String, HttpException> {

  let user_collection = mongo.collection::<User>("users");


  // check if psw is valid
  let _user = user_collection.find_one(doc! {"user_email": &user_email}).await.unwrap();

  if _user.clone().is_none() {
    return Err( HttpException {
      status: Status::Unauthorized,
      message: "There is no such user".to_string(),
    });
  }
  let user = _user.unwrap(); // unwrap after find

  let _psw_is_valid = verify_psw(&user_password, &user.hashed_password, &config.psw_secret);
  if _psw_is_valid == false {
    return Err( HttpException {
      status: Status::Unauthorized,
      message: "Password or email is invalid".to_string(),
    });
  }

  // create token
  let should_expire_at = chrono::Local::now() + chrono::TimeDelta::days(ACCESS_TOKEN_LIVES);
  let raw_token = AccessToken {
    this_jwt_epoch: config.jwt_epoch,
    this_jwt_expires_at: should_expire_at.to_rfc2822(),
    this_user_id: user.user_id,
  };

  let ready_token = jsonwebtoken::encode(
    &JwtHeader::default(),
    &raw_token,
    &EncodingKey::from_secret( config.jwt_secret.as_bytes() )
  ).unwrap();

  Ok(ready_token)
}









pub async fn get_current_user(user: User) -> Result<UserDTO, HttpException> {

  Ok( UserDTO {
    user_id: user.user_id,
    user_name: user.user_name,
    user_email: user.user_email,
    verified: user.verified
  })
}








pub async fn get_user_image(user: User, config: &State<ConfigModel>) -> Result<PathBuf, HttpException> {

  let img_file_name = "avatar_".to_owned() + &user.user_id;
  let user_img_path = Path::new( &config.storage_path ).join( img_file_name );

  return Ok(user_img_path);
}









pub async fn update_user_image(user: User, mut file: Form<TempFile<'_>>, config: &State<ConfigModel>) -> Result<(), HttpException> {

  // if file is not image
  let content_type = file.content_type().unwrap_or(&ContentType::Plain).to_string();
  if content_type.split('/').next().unwrap_or("") != "image" {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "Provided file is not an image".to_string()
    });
  }
  

  let img_file_name = "avatar_".to_owned() + &user.user_id;
  let user_img_path = Path::new( &config.storage_path ).join( img_file_name );

  let _ = fs::remove_file(&user_img_path);


  match fs::copy(file.path().unwrap(), user_img_path) {
  /*match file.persist_to(user_img_path).await {*/ 
      
      Ok(_) => { return Ok(()); },

      Err(e) => {
        println!("{}", e);
        return Err( HttpException {
          status: Status::BadRequest,
          message: "Error while updating user image".to_string(),
        });
      }
  };

}