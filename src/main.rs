use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/api/check/{cep}")]
async fn check(_cep: web::Path<String>) -> impl Responder {

    HttpResponse::Ok().body("Bogdan")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(check))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
