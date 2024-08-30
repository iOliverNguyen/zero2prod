use std::net::TcpListener;

use actix_web::{
    dev::{Server, ServiceFactory, ServiceRequest, ServiceResponse},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde_json::json;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| create_app()).listen(listener)?.run();
    Ok(server)
}

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .route("/health", web::get().to(health))
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
}

pub async fn health(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or_else(|| "World");
    format!("Hello {}", name)
}
