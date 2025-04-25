
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
  pub this_jwt_epoch: i32,
  pub this_jwt_expires_at: String, // will be converted to chrono::DateTime
  pub this_user_id: String,
}



// not used rn
#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
  pub this_jwt_expires_at: String, // will be converted to chrono::DateTime
}