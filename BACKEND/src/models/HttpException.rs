
use rocket::http::Status;



#[derive(Debug)]
pub struct HttpException {
  pub status: Status,
  pub message: String,
}