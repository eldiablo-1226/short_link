use actix_web::*;
use actix_web::web::Redirect;
use uuid::Uuid;

use crate::models::*;
use crate::AppState;
use crate::repository::ShortLinkRepository;

#[get("/{code}")]
async fn redirect(code: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query_result = ShortLinkRepository::get_url_by_code(&code, &data.db).await;

    match query_result {
        Some(c) => Redirect::to(c.original_url),
        None => Redirect::to(format!("{:?}/notfound/", data.domain))
    }
}

#[post("/shorter")]
async fn create_short_link(body: web::Json<InserShortLink>, data: web::Data<AppState>) -> impl Responder {
    let query_result = ShortLinkRepository::get_url_by_url(&body.url, &data.db).await;

    let code = match query_result {
        Some(c) => c.code,
        None => {
            let new_code = Uuid::new_v4().simple().encode_lower(&mut Uuid::encode_buffer()).to_string();
            ShortLinkRepository::insert_short_link(&crate::repository::InserShortLink { code: new_code.clone(), url: body.url.clone(), tag: body.tag.clone() }, &data.db).await;
            new_code
        }
    };

    web::Json(InserShortLinkResult{ url: format!("{:}/{:}", data.domain, code) })
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(redirect)
        .service(create_short_link);

    conf.service(scope);
}