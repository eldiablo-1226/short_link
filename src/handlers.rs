use actix_web::*;

use crate::models::*;
use crate::AppState;

async fn redirect(code: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {code}"))
}


async fn create_short_link(body: web::Json<InserShortLink>, data: web::Data<AppState>) -> impl Responder {
    
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .route("/{code}", web::get().to(redirect))
        .route("/shorter", web::post().to(create_short_link))        ;

    conf.service(scope);
}