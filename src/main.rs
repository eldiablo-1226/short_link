use actix_web::*;
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};

#[get("/{code}")]
async fn redirect(code: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {code}"))
}


#[post("/echo")]
async fn create_short_link(request: web::Json<InserShortLink>) -> impl Responder {

    web::Json(request)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InserShortLink{
    url: String,
    tag: Option<String>
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
            .service(redirect)
            .service(create_short_link)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}