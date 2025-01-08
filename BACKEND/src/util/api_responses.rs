
use rocket::{Request, Response};
use rocket::response::{Responder, Result as ResponseResult};
use rocket::serde::json::Value as JsonValue;
use rocket::http::{Status, ContentType};




#[derive(Debug)]
pub struct ApiJsonResponse {
  pub value: JsonValue,
  pub status: Status,
}
impl<'r> Responder<'r, 'r> for ApiJsonResponse {
  fn respond_to(self, req: &Request) -> ResponseResult<'r> {
    Response::build_from(self.value.respond_to(&req).unwrap())
      .status(self.status)
      .header(ContentType::JSON)
      .ok()
  }
}







#[derive(Debug)]
pub struct ApiTextResponse {
  pub value: String,
  pub status: Status,
}
impl<'r> Responder<'r, 'r> for ApiTextResponse {
  fn respond_to(self, req: &Request) -> ResponseResult<'r> {
    Response::build_from(self.value.respond_to(&req).unwrap())
      .status(self.status)
      .header(ContentType::Text)
      .ok()
  }
}
