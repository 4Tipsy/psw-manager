
use rocket::async_trait;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

// modules
use crate::config::ConfigModel;



pub struct CorsFairing;



#[async_trait]
impl Fairing for CorsFairing {

  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to responses",
      kind: Kind::Response
    }
  }


  async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {

    let config: &ConfigModel = req.rocket().state().unwrap();


    
    for host in &config.cors_allowed_hosts {
      res.set_header(Header::new("Access-Control-Allow-Origin", host));
    }
    res.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PATCH, DELETE"));
    res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Content-Length"));
    res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  }

}