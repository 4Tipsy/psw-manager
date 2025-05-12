
use rocket::async_trait;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

// modules
use crate::API_VERSION;



pub struct SomeHeadersFairing;



#[async_trait]
impl Fairing for SomeHeadersFairing {

  fn info(&self) -> Info {
    Info {
      name: "Add some headers",
      kind: Kind::Response
    }
  }


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