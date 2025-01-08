
use bcrypt::{self, DEFAULT_COST};





pub fn hash_psw(psw: &str, secret: &str) -> String {

  let psw_with_secret = String::new() + psw + secret; // i know -_-

  bcrypt::hash(psw_with_secret, DEFAULT_COST).unwrap()
}





pub fn verify_psw(psw: &str, hashed_psw: &str, secret: &str) -> bool {

  let psw_with_secret = String::new() + psw + secret; // i know -_-

  bcrypt::verify(psw_with_secret, hashed_psw).unwrap()
}