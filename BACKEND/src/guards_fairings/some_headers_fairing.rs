
use rocket::async_trait;
use rocket::http::Header;
use rocket::{Request, Response, Data};
use rocket::fairing::{Fairing, Info, Kind};

use std::panic;

// modules
use crate::API_VERSION;



pub struct SomeHeadersFairing;



#[async_trait]
impl Fairing for SomeHeadersFairing {

  fn info(&self) -> Info {
    Info {
      name: "Add some headers",
      kind: Kind::Request | Kind::Response
    }
  }




  /* if ip header */
  async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {

    if req.headers().contains("X-Real-IP") {
      // will abort request to avoid ip spoofing
      panic!("X-Real-IP header is not allowed");
    }
  }




  /* add some headers */
  async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {

    let req_ip: String = match req.client_ip() {
      Some(_ip) => _ip.to_string(),
      None => "std::ptr::null".to_string()
    };

    let api_version_str: String = format!("PswManagerAPI: '{}'", API_VERSION);


    // set headers
    res.set_header(Header::new("X-Api-Version", api_version_str));
    res.set_header(Header::new("X-Your-Ip", req_ip));
  }
}