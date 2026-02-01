use sqlx::postgres::PgPool;

use super::types::{LangInfo, CardType, DeckSummary, CardSummary, AddCardForm, CardWithId, DeckInfo};

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
pub fn create_slug(name: &str) -> String {
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
                   d.slug,
                   d.description,
                   l.name as language_name,
                   coalesce(count(distinct ctd.card_id), 0)::int as total_cards
            from card_decks as d
            join languages as l on l.id = d.language_id
            left join card_to_deck as ctd on ctd.deck_id = d.id
            where l.slug = $1
            group by d.id, d.name, d.slug, d.description, l.name
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
                   d.slug,
                   d.description,
                   l.name as language_name,
                   coalesce(count(distinct ctd.card_id), 0)::int as total_cards
            from card_decks as d
            join languages as l on l.id = d.language_id
            left join card_to_deck as ctd on ctd.deck_id = d.id
            group by d.id, d.name, d.slug, d.description, l.name
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
               c.translation,
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

/// Insert a new card deck into the database
/// Returns the created deck's slug for redirect
pub async fn create_deck(
    language_slug: String,
    name: String,
    description: Option<String>,
    pool: &PgPool
) -> Result<String, String> {
    // 1. Look up language_id from slug
    let language_result = sqlx::query!(
        "SELECT id FROM languages WHERE slug = $1",
        language_slug
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to look up language '{}': {}", language_slug, e);
        "Failed to query language from database".to_string()
    })?;

    let language_id = match language_result {
        Some(record) => record.id,
        None => {
            log::warn!("Language with slug '{}' not found", language_slug);
            return Err(format!("Language '{}' not found", language_slug));
        }
    };

    // 2. Create slug from deck name
    let slug = create_slug(&name);

    // 3. Insert deck
    let result = sqlx::query!(
        "INSERT INTO card_decks (name, slug, description, language_id) VALUES ($1, $2, $3, $4)",
        name,
        slug,
        description,
        language_id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            log::info!("Successfully created deck: {} (slug: {})", name, slug);
            Ok(slug)
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    log::warn!("Deck with slug '{}' already exists in language '{}'", slug, language_slug);
                    return Err("A deck with a similar name already exists in this language".to_string());
                }
            }
            log::error!("Failed to insert deck '{}': {}", name, e);
            Err("Failed to create deck in database".to_string())
        }
    }
}

/// Get deck information by slug for the edit page header
pub async fn get_deck_info(
    language_slug: &str,
    deck_slug: &str,
    pool: &PgPool
) -> Result<DeckInfo, String> {
    sqlx::query_as::<_, DeckInfo>(r#"
        SELECT d.id, d.name, d.slug, d.description,
               l.name as language_name, l.slug as language_slug
        FROM card_decks d
        JOIN languages l ON l.id = d.language_id
        WHERE l.slug = $1 AND d.slug = $2
    "#)
    .bind(language_slug)
    .bind(deck_slug)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to fetch deck info for {}/{}: {}", language_slug, deck_slug, e);
        "Unable to query deck from database".to_string()
    })?
    .ok_or_else(|| "Deck not found".to_string())
}

/// Get cards for a language that are NOT already in the specified deck
/// Optionally filter by card type
pub async fn list_available_cards_for_deck(
    deck_id: i32,
    language_slug: &str,
    type_filter: Option<i32>,
    page: i32,
    pool: &PgPool
) -> Result<(Vec<CardWithId>, bool), String> {
    if page <= 0 {
        return Err("Page must be greater than 0".to_string());
    }

    let start = (page - 1) * PAGE_SIZE;
    let limit = PAGE_SIZE + 1;

    let mut result = if let Some(type_id) = type_filter {
        sqlx::query_as::<_, CardWithId>(r#"
            SELECT c.id, c.target, c.translation, c.hint, ct.type_name
            FROM cards c
            JOIN card_types ct ON ct.id = c.type_id
            JOIN languages l ON l.id = c.language_id
            WHERE l.slug = $1
              AND c.type_id = $2
              AND c.id NOT IN (
                  SELECT card_id FROM card_to_deck WHERE deck_id = $3
              )
            ORDER BY c.id DESC
            LIMIT $4 OFFSET $5
        "#)
        .bind(language_slug)
        .bind(type_id)
        .bind(deck_id)
        .bind(limit)
        .bind(start)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, CardWithId>(r#"
            SELECT c.id, c.target, c.translation, c.hint, ct.type_name
            FROM cards c
            JOIN card_types ct ON ct.id = c.type_id
            JOIN languages l ON l.id = c.language_id
            WHERE l.slug = $1
              AND c.id NOT IN (
                  SELECT card_id FROM card_to_deck WHERE deck_id = $2
              )
            ORDER BY c.id DESC
            LIMIT $3 OFFSET $4
        "#)
        .bind(language_slug)
        .bind(deck_id)
        .bind(limit)
        .bind(start)
        .fetch_all(pool)
        .await
    }
    .map_err(|e| {
        log::error!("Unable to fetch available cards: {}", e);
        "Unable to read cards from database".to_string()
    })?;

    let has_next = result.len() > PAGE_SIZE as usize;
    if has_next {
        result.pop();
    }
    Ok((result, has_next))
}

/// Add a card to a deck (insert into junction table)
pub async fn add_card_to_deck(
    card_id: i32,
    deck_slug: &str,
    language_slug: &str,
    pool: &PgPool
) -> Result<(), String> {
    // Look up deck_id from slugs
    let deck_result = sqlx::query!(
        r#"SELECT d.id FROM card_decks d
           JOIN languages l ON l.id = d.language_id
           WHERE d.slug = $1 AND l.slug = $2"#,
        deck_slug,
        language_slug
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to look up deck {}/{}: {}", language_slug, deck_slug, e);
        "Failed to query deck from database".to_string()
    })?;

    let deck_id = match deck_result {
        Some(record) => record.id,
        None => {
            log::warn!("Deck {}/{} not found", language_slug, deck_slug);
            return Err("Deck not found".to_string());
        }
    };

    // Insert into junction table
    let result = sqlx::query!(
        "INSERT INTO card_to_deck (card_id, deck_id) VALUES ($1, $2)",
        card_id,
        deck_id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            log::info!("Added card {} to deck {}", card_id, deck_slug);
            Ok(())
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    log::warn!("Card {} already in deck {}", card_id, deck_slug);
                    return Err("Card is already in this deck".to_string());
                }
                if db_err.is_foreign_key_violation() {
                    log::warn!("Invalid card {} or deck {}", card_id, deck_slug);
                    return Err("Invalid card or deck".to_string());
                }
            }
            log::error!("Failed to add card to deck: {}", e);
            Err("Failed to add card to deck".to_string())
        }
    }
}

/// Get deck information by slug without requiring language slug
pub async fn get_deck_by_slug(
    deck_slug: &str,
    pool: &PgPool
) -> Result<DeckInfo, String> {
    sqlx::query_as::<_, DeckInfo>(r#"
        SELECT d.id, d.name, d.slug, d.description,
               l.name as language_name, l.slug as language_slug
        FROM card_decks d
        JOIN languages l ON l.id = d.language_id
        WHERE d.slug = $1
    "#)
    .bind(deck_slug)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::error!("Failed to fetch deck info for {}: {}", deck_slug, e);
        "Unable to query deck from database".to_string()
    })?
    .ok_or_else(|| "Deck not found".to_string())
}

/// Get cards that belong to a specific deck
pub async fn list_cards_for_deck(
    deck_slug: &str,
    page: i32,
    pool: &PgPool
) -> Result<(Vec<CardSummary>, bool), String> {
    if page <= 0 {
        return Err("Page must be greater than 0".to_string());
    }

    let start = (page - 1) * PAGE_SIZE;
    let limit = PAGE_SIZE + 1;

    let mut result = sqlx::query_as::<_, CardSummary>(r#"
        SELECT c.target,
               c.translation,
               c.hint,
               c.examples,
               c.additional_info,
               ct.type_name,
               l.name as language
        FROM cards c
        JOIN card_types ct ON ct.id = c.type_id
        JOIN languages l ON l.id = c.language_id
        JOIN card_to_deck ctd ON ctd.card_id = c.id
        JOIN card_decks d ON d.id = ctd.deck_id
        WHERE d.slug = $1
        ORDER BY c.id DESC
        LIMIT $2 OFFSET $3
        "#)
        .bind(deck_slug)
        .bind(limit)
        .bind(start)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch cards for deck {}: {}", deck_slug, e);
            "Unable to read cards from database".to_string()
        })?;

    let has_next = result.len() > PAGE_SIZE as usize;
    if has_next {
        result.pop();
    }
    Ok((result, has_next))
}
