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
    pub slug: String,
    pub description: Option<String>,
    pub language_name: String,
    pub language_slug: String,
    pub total_cards: i32,
}

/// Represents a card for display in list views
#[derive(FromRow, Debug)]
pub struct CardSummary {
    pub target: String,
    pub translation: String,
    pub hint: Option<String>,
    pub examples: Option<String>,
    pub additional_info: Option<String>,
    #[sqlx(rename = "type_name")]
    pub card_type: String,
    pub language: String,
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

/// Form data for adding a new card
#[derive(Debug, Deserialize)]
pub struct AddCardForm {
    pub type_id: i32,
    pub language_slug: String,
    pub target: String,
    pub translation: String,
    pub hint: Option<String>,
    pub examples: Option<String>,
    pub additional_info: Option<String>,
}

/// Form data for creating a new deck
#[derive(Debug, Deserialize)]
pub struct AddDeckForm {
    pub language_slug: String,
    pub name: String,
    pub description: Option<String>,
}

/// Form data for adding a card to a deck (preserves pagination state)
#[derive(Debug, Deserialize)]
pub struct AddCardToDeckForm {
    pub card_id: i32,
    pub deck_slug: String,
    pub language_slug: String,
    pub page: i32,
    pub type_filter: Option<i32>,
}

/// Card with ID for edit deck view
#[derive(FromRow, Debug)]
pub struct CardWithId {
    pub id: i32,
    pub target: String,
    pub translation: String,
    pub hint: Option<String>,
    #[sqlx(rename = "type_name")]
    pub card_type: String,
}

/// Deck info for edit page header
#[derive(FromRow, Debug)]
pub struct DeckInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub language_name: String,
    pub language_slug: String,
}

/// Information on a user retrieved from the DB
#[derive(FromRow, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub uuid: String,
    pub pw_hash: String,
}

/// JWT claims embedded in the auth token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub uuid: String,
    pub exp: usize,
}

/// Form data submitted from the login page
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

/// Form data submitted from the create user page
#[derive(Debug, Deserialize)]
pub struct CreateUserForm {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

/// Authenticated user info extracted from a valid JWT, available in request extensions
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub username: String,
    pub uuid: String,
}
