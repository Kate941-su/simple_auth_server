use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Your condition is healthy";
    return HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server is starting!");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run().await?;

    Ok(())
}
