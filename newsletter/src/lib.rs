use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde_json::json;

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| create_app())
        .bind("127.0.0.1:8080")?
        .run()
        .await
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
