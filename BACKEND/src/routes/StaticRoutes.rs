
use rocket::get;
use rocket::fs::NamedFile;

use std::path::Path;





#[get("/docs-swagger")]
pub async fn send_docs_swagger() -> Option<NamedFile> {
  let path = Path::new("./data/swagger.html");
  return NamedFile::open(path).await.ok();
}

#[get("/docs-redoc")]
pub async fn send_docs_redoc() -> Option<NamedFile> {
  let path = Path::new("./data/redoc.html");
  return NamedFile::open(path).await.ok();
}




#[get("/openapi.yaml")]
pub async fn send_openapi() -> Option<NamedFile> {
  let path = Path::new("./data/openapi.yaml");
  return NamedFile::open(path).await.ok();
}