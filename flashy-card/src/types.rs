use sqlx::FromRow;
use serde::{Serialize, Deserialize};

/// Overview of information on a language for list views.
#[derive(FromRow, Debug)]
pub struct LangInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

/// A Card type to use for reference
#[derive(FromRow, Debug, Serialize)]
pub struct CardType {
    pub id: i32,
    pub type_name: String,
}

/// Represents a summary of a card deck for display
#[derive(FromRow, Debug)]
pub struct DeckSummary {
    pub name: String,
    pub description: Option<String>,
    pub language_name: String,
    pub total_cards: i32,
}

/// Response for a list of card types request to the API
#[derive(Debug, Serialize)]
pub struct CardTypeList {
    pub card_types: Vec<CardType>,
}


/// Error response from the API
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Form data for adding a new language
#[derive(Debug, Deserialize)]
pub struct AddLanguageForm {
    pub name: String,
    pub description: Option<String>,
}
