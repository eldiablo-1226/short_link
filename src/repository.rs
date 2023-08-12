use sqlx::{PgPool, Error};

use crate::models::*;

struct ShortLinkRepository;

impl ShortLinkRepository {
    async fn get_url_by_code(code: &String, con: &PgPool) -> Result<String, Error> {
        let code = sqlx::query_as(r#"DELETE FROM notes WHERE id = ?"#)
            .bind(code)
            .fetch_optional(executor)
            .fetch_one(&data.db)
            .await
        
    }
}