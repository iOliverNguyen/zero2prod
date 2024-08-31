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
        .route("/", web::get().to(greet))
        .route("/health", web::get().to(health))
        .route("/{name}", web::get().to(greet))
        .route("/subscribe", web::post().to(subscribe))
}

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    pub name: String,
    pub email: String,
}

pub async fn health(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or_else(|| "World");
    format!("Hello {}", name)
}

pub async fn subscribe(form: web::Form<SubscribeForm>) -> impl Responder {
    match (form.name.is_empty(), form.email.is_empty()) {
        (true, true) => {
            return HttpResponse::BadRequest().json(json!({ "error": "missing name and email" }));
        }
        (false, true) => {
            return HttpResponse::BadRequest().json(json!({ "error": "missing email" }));
        }
        (true, false) => {
            return HttpResponse::BadRequest().json(json!({ "error": "missing name" }));
        }
        (false, false) => HttpResponse::Ok()
            .json(json!({ "name": form.name, "email": form.email, "status": "ok" })),
    }
}
