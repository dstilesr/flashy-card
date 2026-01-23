use sqlx::postgres::PgPool;

use super::types::{LangInfo, CardType, DeckSummary, CardSummary, AddCardForm};

const PAGE_SIZE: i32 = 10;

/// Fetch a page of language records from the database.
pub async fn get_languages(page: i32, pool: &PgPool) -> Result<(Vec<LangInfo>, bool), String> {
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

/// Get a list of all card types.
pub async fn list_card_types(pool: &PgPool) -> Result<Vec<CardType>, String> {
    sqlx::query_as::<_, CardType>("select * from card_types;")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch card types: {}", e);
            "Unable to read card types from database".to_string()
        })
}

/// Get the language name from a slug
pub async fn get_language_name(slug: &str, pool: &PgPool) -> Result<String, String> {
    let result = sqlx::query!(
        "SELECT name FROM languages WHERE slug = $1",
        slug
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Database error fetching language with slug '{}': {}", slug, e);
        "Unable to query language from database".to_string()
    })?;

    match result {
        Some(record) => Ok(record.name),
        None => {
            log::warn!("Language with slug '{}' not found", slug);
            Err(format!("Language '{}' not found", slug))
        }
    }
}

/// Create a URL-safe slug from a language name
fn create_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Insert a new language into the database
pub async fn add_language(name: String, description: Option<String>, pool: &PgPool) -> Result<(), String> {
    let slug = create_slug(&name);
    let result = sqlx::query!(
        "INSERT INTO languages (name, slug, description) VALUES ($1, $2, $3)",
        name,
        slug,
        description
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            log::info!("Successfully added language: {} (slug: {})", name, slug);
            Ok(())
        }
        Err(e) => {
            // Check if it's a unique constraint violation
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    log::warn!("Language with slug '{}' already exists", slug);
                    return Err(format!("A language with a similar name already exists (slug: {})", slug));
                }
            }
            log::error!("Failed to insert language '{}': {}", name, e);
            Err("Failed to create language in database".to_string())
        }
    }
}


/// Get the list of card decks. If a language slug is given, return only decks
/// for that language.
pub async fn list_decks(
    language_slug: Option<String>,
    page: i32,
    pool: &PgPool
) -> Result<(Vec<DeckSummary>, bool), String> {
    if page <= 0 {
        return Err("Page must be greater than 0".to_string());
    }

    let start = (page - 1) * PAGE_SIZE;
    let limit = PAGE_SIZE + 1;

    let query = if let Some(slug) = language_slug {
        sqlx::query_as::<_, DeckSummary>(r#"
            select d.name,
                   d.description,
                   l.name as language_name,
                   coalesce(count(distinct ctd.card_id), 0)::int as total_cards
            from card_decks as d
            join languages as l on l.id = d.language_id
            left join card_to_deck as ctd on ctd.deck_id = d.id
            where l.slug = $1
            group by d.id, d.name, d.description, l.name
            order by d.id desc
            limit $2
            offset $3;
            "#)
            .bind(slug)
            .bind(limit)
            .bind(start)
    } else {
        sqlx::query_as::<_, DeckSummary>(r#"
            select d.name,
                   d.description,
                   l.name as language_name,
                   coalesce(count(distinct ctd.card_id), 0)::int as total_cards
            from card_decks as d
            join languages as l on l.id = d.language_id
            left join card_to_deck as ctd on ctd.deck_id = d.id
            group by d.id, d.name, d.description, l.name
            order by d.id desc
            limit $1
            offset $2;
            "#)
            .bind(limit)
            .bind(start)
    };

    let mut result = query.fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch decks: {}", e);
            "Unable to read decks from database".to_string()
        })?;

    let has_next = result.len() > PAGE_SIZE as usize;
    if has_next {
        result.pop();
    }
    Ok((result, has_next))
}

/// Get the list of cards for a given language.
pub async fn list_cards(
    language_slug: String,
    page: i32,
    pool: &PgPool
) -> Result<(Vec<CardSummary>, bool), String> {
    if page <= 0 {
        return Err("Page must be greater than 0".to_string());
    }

    let start = (page - 1) * PAGE_SIZE;
    let limit = PAGE_SIZE + 1;

    let mut result = sqlx::query_as::<_, CardSummary>(r#"
        select c.target,
               c.hint,
               c.examples,
               c.additional_info,
               ct.type_name,
               l.name as language
        from cards as c
        join card_types as ct on ct.id = c.type_id
        join languages as l on l.id = c.language_id
        where l.slug = $1
        order by c.id desc
        limit $2
        offset $3;
        "#)
        .bind(language_slug)
        .bind(limit)
        .bind(start)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch cards: {}", e);
            "Unable to read cards from database".to_string()
        })?;

    let has_next = result.len() > PAGE_SIZE as usize;
    if has_next {
        result.pop();
    }
    Ok((result, has_next))
}

/// Insert a new card into the database
pub async fn add_card(form: AddCardForm, pool: &PgPool) -> Result<(), String> {
    // First, look up the language_id from the slug
    let language_result = sqlx::query!(
        "SELECT id FROM languages WHERE slug = $1",
        form.language_slug
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to look up language '{}': {}", form.language_slug, e);
        "Failed to query language from database".to_string()
    })?;

    let language_id = match language_result {
        Some(record) => record.id,
        None => {
            log::warn!("Language with slug '{}' not found", form.language_slug);
            return Err(format!("Language '{}' not found", form.language_slug));
        }
    };

    // Insert the card
    let result = sqlx::query!(
        r#"INSERT INTO cards (type_id, language_id, target, translation, hint, examples, additional_info)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        form.type_id,
        language_id,
        form.target,
        form.translation,
        form.hint,
        form.examples,
        form.additional_info
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            log::info!("Successfully added card '{}' for language '{}'", form.target, form.language_slug);
            Ok(())
        }
        Err(e) => {
            // Check if it's a foreign key violation (invalid type_id)
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_foreign_key_violation() {
                    log::warn!("Invalid type_id {} or language_id {}", form.type_id, language_id);
                    return Err("Invalid card type or language".to_string());
                }
            }
            log::error!("Failed to insert card '{}': {}", form.target, e);
            Err("Failed to create card in database".to_string())
        }
    }
}
