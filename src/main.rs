use actix_web::*;
use actix_web::middleware::Logger;
use actix_cors::Cors;
use config::Config;

use sqlx::postgres::{PgPool, PgPoolOptions};

mod handlers;
mod models;
mod repository;
mod auth;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    domain: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./Settings.toml"))
        .build()
        .unwrap();

    if settings.get_string("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Init PG
    let database_url = settings.get_string("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let state = AppState{
        db: pool,
        domain: settings.get_string("DOMAIN").expect("DOMAIN must be set")
    };

    auth::init(settings.clone());

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
