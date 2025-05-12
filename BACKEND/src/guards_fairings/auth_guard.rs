
use std::collections::HashSet;

use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use rocket::async_trait;
use rocket::State;

use jsonwebtoken::{self, DecodingKey, Validation};


use mongodb::Database;
use mongodb::bson::doc;
use chrono;

// modules
use crate::models::user_model::User;
use crate::models::access_token::AccessToken;
use crate::config::ConfigModel;




pub struct AuthGuard {
  pub value: User,
}




#[async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
  type Error = String;


  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, String> {

    let config: &State<ConfigModel> = req.guard().await.unwrap();
    let mongo: &State<Database> = req.guard().await.unwrap();
    let _if_err: String = format!("{} {}", req.method().to_string(), req.uri().to_string()); // will be returned on error // log purpose


    // get cookie
    let access_cookie = req.cookies().get("psw-manager.access_token");
    if access_cookie.is_none() {
      return Outcome::Error((Status::Unauthorized, _if_err)); // RETURN
    }


    // proceed token
    let token = access_cookie.unwrap().value();
    let secret = DecodingKey::from_secret( config.jwt_secret.as_bytes() );
    let mut _validation = Validation::default();
    _validation.required_spec_claims = HashSet::new();
    _validation.validate_exp = false;
    match jsonwebtoken::decode::<AccessToken>(token, &secret, &_validation) {


      Ok(parsed_token) => {

        // check if token expired
        let time_now = chrono::prelude::Local::now();
        let time_expires_at = chrono::DateTime::parse_from_rfc2822(
          &parsed_token.claims.this_jwt_expires_at
        ).unwrap();
        if time_now > time_expires_at {
          return Outcome::Error((Status::Unauthorized, _if_err)); // RETURN
        }

        // check if ip is matching
        if parsed_token.claims.this_device_ip != req.client_ip().unwrap().to_string() {
          return Outcome::Error((Status::ProxyAuthenticationRequired, _if_err)); // RETURN
        }

        // check if user exists / is verified
        let user: Option<User> = mongo.collection::<User>("users")
          .find_one(doc! {"user_id": &parsed_token.claims.this_user_id}).await.unwrap();

        if user.is_none() {
          return Outcome::Error((Status::Unauthorized, _if_err)); // RETURN
        }
        if user.clone().unwrap().verified == false {
          return Outcome::Error((Status::Unauthorized, _if_err)); // RETURN
        }

        // if ok
        Outcome::Success( AuthGuard {value: user.unwrap()} ) // RETURN IF OK
      },



      // on jwt verify failure
      Err(_) => {
        return Outcome::Error((Status::Unauthorized, _if_err)); // RETURN
      }


    }
    
  }
}