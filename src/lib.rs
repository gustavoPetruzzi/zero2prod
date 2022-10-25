use std::net::TcpListener;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(address)?
        .run();
    Ok(server)
}