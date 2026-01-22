use sqlx::postgres;

use super::types::LangInfo;

const PAGE_SIZE: i32 = 10;

/// Fetch a page of language records from the database.
pub async fn get_languages(page: i32, pool: &postgres::PgPool) -> Result<(Vec<LangInfo>, bool), String> {
    let limit = PAGE_SIZE + 1;
    let offset = (page - 1) * PAGE_SIZE;

    let mut out = sqlx::query_as::<_, LangInfo>("select * from languages limit $1 offset $2;")
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch languages: {}", e);
            "Unable to read languages from database".to_string()
        })?;

    let has_next = out.len() > PAGE_SIZE as usize;
    if has_next {
        out.pop();
    }
    Ok((out, has_next))
}
