use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cached::proc_macro::cached;
use cached::SizedCache;

#[get("/api/check/{cep}")]
async fn check(cep: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(get_cep(cep).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(check))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cached(
    type = "SizedCache<String, String>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}", cep) }"#
)]
async fn get_cep(cep: web::Path<String>) -> String {
    let response = reqwest::get(format!("https://viacep.com.br/ws/{}/json/", cep))
    .await
    .unwrap()
    .text_with_charset("utf-8")
    .await;

    match response {
        Ok(result) => result,
        _ => format!("Not found {}!", cep),
    }
}