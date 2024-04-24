use actix_web::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("body")
}

#[actix_web::post("/")]
async fn echo(req_body: web::Json<Info>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("username: {}", req_body.username)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let _json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .service(hello)
            .service(echo)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
