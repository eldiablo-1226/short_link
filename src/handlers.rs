use actix_web::*;
use actix_web::web::Redirect;

use crate::models::*;
use crate::AppState;
use crate::repository::ShortLinkRepository;

#[get("/{code}")]
async fn redirect(code: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query_result = ShortLinkRepository::get_url_by_code(&code, &data.db).await;

    match query_result {
        Some(c) => Redirect::to(c.original_url),
        None => Redirect::to("https://localhost:8080/notfound.")
    }
}

#[post("/shorter")]
async fn create_short_link(body: web::Json<InserShortLink>, data: web::Data<AppState>) -> impl Responder {
    let query_result = ShortLinkRepository::get_url_by_url(&body.url, &data.db).await;

    let code = match query_result {
        Some(c) => c.code,
        None => {
            let new_code = uuid::Uuid::new_v4().to_string();
            ShortLinkRepository::insert_short_link(&crate::repository::InserShortLink { code: new_code.clone(), url: body.url.clone(), tag: body.tag.clone() }, &data.db).await;
            new_code
        }
    };

    web::Json(InserShortLinkResult{ url: format!("https://localhost:8080/{code}") })
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(redirect)
        .service(create_short_link);

    conf.service(scope);
}