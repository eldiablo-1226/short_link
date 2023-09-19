use actix_web::web::Redirect;
use actix_web::*;
use uuid::Uuid;

use crate::models::*;
use crate::repository::ShortLinkRepository;
use crate::AppState;

#[get("/{code}")]
async fn redirect(code: web::Path<String>, data: web::Data<AppState>) -> Redirect
{
    let query_result = ShortLinkRepository::get_url_by_code(&code, &data.db).await;

    match query_result 
    {
        Some(c) => Redirect::to(c.original_url),
        None => Redirect::new("/", "/notfound/"),
    }
}

#[post("/shorter")]
async fn create_short_link(
    body: web::Json<InserShortLink>,
    data: web::Data<AppState>,
) -> web::Json<InserShortLinkResult>
{
    let query_result = ShortLinkRepository::get_code_by_url(&body.url, &data.db).await;

    let code = match query_result 
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

    web::Json(InserShortLinkResult{ url: format!("{:}/{:}", data.domain, code) })
}

pub fn config(conf: &mut web::ServiceConfig) 
{
    let scope = web::scope("")
        .service(redirect)
        .service(create_short_link);

    conf.service(scope);
}