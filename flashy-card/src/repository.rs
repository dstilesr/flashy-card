use sqlx::postgres;

use super::types::LangInfo;

const PAGE_SIZE: i32 = 10;

/// Fetch a page of language records from the database.
pub async fn get_languages(page: i32, pool: &postgres::PgPool) -> Result<(Vec<LangInfo>, bool), String> {
    let mut out = sqlx::query_as::<_, LangInfo>("select * from languages limit ? offset ?;")
        .bind(page * PAGE_SIZE + 1)
        .bind(page - 1)
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
