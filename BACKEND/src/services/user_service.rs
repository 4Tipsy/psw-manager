
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
use crate::models::user_model::{User, NewUserDTO, UserDTO};
use crate::models::access_token::AccessToken;
use crate::models::http_exception::HttpException;
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
      new_user_id = "U-".to_owned() + gen_simple_hash(11).as_ref();

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
  let _img_file_name = "avatar__".to_owned() + &new_user_id + "__" + ".jpg";
  let user_img_path = Path::new( &config.storage_path ).join( &_img_file_name );
  let _ = fs::copy("./data/default_user_image.jpg", user_img_path);


  // insert new user
  let ready_user = User {
    user_id: new_user_id,
    user_name: new_user.user_name,
    user_email: new_user.user_email,
    hashed_password: hash_psw(&new_user.password, &config.psw_secret),
    user_image: _img_file_name,
    verified: false,
  };
  let _ = user_collection.insert_one(ready_user).await.expect("Error while inserting");


  Ok(())
}













pub async fn get_user_token(user_email: &str, user_password: &str, req_ip: &str, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<String, HttpException> {

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
    this_device_ip: req_ip.to_string(),
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
    user_image: user.user_image,
    verified: user.verified
  })
}








pub async fn get_user_image(user: User, config: &State<ConfigModel>) -> Result<PathBuf, HttpException> {

  let user_img_path: PathBuf = Path::new( &config.storage_path ).join( &user.user_image );

  return Ok(user_img_path);
}








#[allow(unused_mut)]
pub async fn update_user_image(user: User, mut file: Form<TempFile<'_>>, mongo: &State<Database>, config: &State<ConfigModel>) -> Result<(), HttpException> {

  // if file is not image
  let content_type = file.content_type().unwrap_or(&ContentType::Plain).to_string();
  if content_type.split('/').next().unwrap_or("") != "image" {
    return Err( HttpException {
      status: Status::BadRequest,
      message: "Provided file is not an image".to_string()
    });
  }
  
  let _e = ".".to_string() + &file.content_type().unwrap().to_string().split('/').last().unwrap_or("unknown"); // get file extension, returns ".ext"
  let user_new_img_name = "avatar__".to_owned() + &user.user_id + "__" + &_e;
  let user_new_img_path = Path::new( &config.storage_path ).join( &user_new_img_name );

  // delete old user image
  let _ = fs::remove_file(Path::new(&config.storage_path).join(&user.user_image));


  match fs::copy(file.path().unwrap(), user_new_img_path) {
  /*match file.persist_to(user_img_path).await {*/ 
      
      Ok(_) => {
        let _ = mongo.collection::<User>("users").find_one_and_update(
          doc! {"user_id": &user.user_id},
          doc! {"$set": doc! {"user_image": &user_new_img_name} }
        ).await.unwrap();
        return Ok(());
      },

      Err(e) => {
        println!("___! {}", e);
        return Err( HttpException {
          status: Status::BadRequest,
          message: "Error while updating user image".to_string(),
        });
      }
  };

}