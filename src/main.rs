use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/api/health")]
async fn health_handler() -> impl Responder {
    const MESSAGE: &str = "Razor API";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}