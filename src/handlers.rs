use actix_web::web::Redirect;
use actix_web::*;
use actix_web::http::StatusCode;
use actix_web_httpauth::middleware::HttpAuthentication;
use uuid::Uuid;
use validator::Validate;

use crate::models::*;
use crate::repository::ShortLinkRepository;
use crate::AppState;
use crate::auth::basic_auth_validator;

#[route("/{code}", method="GET", method="HEAD")]
async fn redirect(code: web::Path<String>, data: web::Data<AppState>) -> Redirect
{
    match ShortLinkRepository::get_url_by_code(&code, &data.db).await
    {
        Some(c) => Redirect::to(c.original_url),
        None => Redirect::new("/", "/notfound"),
    }
}

#[post("/shorter")]
async fn create_short_link(body: web::Json<InsertShortLink>, data: web::Data<AppState>) -> impl Responder
{
    // Validate request
    if let Err(err) = body.validate(){
        return HttpResponse::BadRequest().json(err);
    }

    // Create short link if not exist
    let code = match ShortLinkRepository::get_code_by_url(&body.url, &data.db).await
    {
        Some(c) => c.code,
        None =>
        {
            let new_code = Uuid::new_v4()
                .simple()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string();

            ShortLinkRepository::insert_short_link(&new_code, &body.url, &body.tag, &data.db).await;

            new_code
        }
    };

    HttpResponse::Ok().json(InsertShortLinkResult { url: format!("{:}/{:}", data.domain, code) })
}

#[get("/notfound")]
async fn not_found_page() -> impl Responder
{
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/404.html"))
}

pub fn config(conf: &mut web::ServiceConfig) 
{
    let auth = HttpAuthentication::basic(basic_auth_validator);

    let auth_scope = web::scope("")
        .wrap(auth)
        .service(create_short_link);

    conf.service(not_found_page).service(redirect).service(auth_scope);
}