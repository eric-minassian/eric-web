use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    message: String,
}

#[get("/hello")]
async fn index() -> Result<impl Responder> {
    let obj = MyObj {
        message: "Hello World".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(web::scope("/api").service(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
