use sqlx::{PgPool, Postgres};

#[derive(Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct GetByCodeResult{
    pub original_url: String
}

#[derive(Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct GetByUrlResult{
    pub code: String
}

#[derive(Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct InserShortLink{
    pub code: String,
    pub url: String,
    pub tag: Option<String>
}


pub struct ShortLinkRepository;

impl ShortLinkRepository {
    pub async fn get_url_by_code(code: &String, conn: &PgPool) -> Option<GetByCodeResult> 
    {
        return sqlx::query_as::<Postgres, GetByCodeResult>(r#"select su.original_url from short_urls as su where su.code = $1 limit 1"#)
            .bind(code)
            .fetch_optional(conn)
            .await
            .expect("Error get short-url by code");
    }

    pub async fn get_url_by_url(url: &String, conn: &PgPool) -> Option<GetByUrlResult> 
    {
        return sqlx::query_as::<Postgres, GetByUrlResult>(r#"select su.code from short_urls as su where su.original_url = $1 limit 1"#)
            .bind(url)
            .fetch_optional(conn)
            .await
            .expect("Error get short-url by url");
    }

    pub async fn insert_short_link(short_link: &InserShortLink, conn: &PgPool) -> sqlx::postgres::PgQueryResult
    {
        return sqlx::query::<Postgres>(r#"insert into short_urls (code, original_url, tag) VALUES ($1, $2, $3)"#)
            .bind(&short_link.code)
            .bind(&short_link.url)
            .bind(&short_link.tag)
            .execute(conn)
            .await
            .expect("Error insert short link");
    }
}