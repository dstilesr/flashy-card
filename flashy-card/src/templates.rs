use askama::Template;
use super::types::{LangInfo, DeckSummary, CardType, CardWithId, DeckInfo, CardSummary};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "create_user.html")]
pub struct CreateUserPage {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage;

#[derive(Template)]
#[template(path = "languages.html")]
pub struct LanguagesPage {
    pub languages: Vec<LangInfo>,
    pub has_next: bool,
    pub page: i32,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub title: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "decks.html")]
pub struct DecksPage {
    pub base_url: String,
    pub decks: Vec<DeckSummary>,
    pub page: i32,
    pub has_next: bool,
}

#[derive(Template)]
#[template(path = "add_card.html")]
pub struct AddCardPage {
    pub language_slug: String,
    pub language_name: String,
    pub card_types: Vec<CardType>,
}

#[derive(Template)]
#[template(path = "create_deck.html")]
pub struct CreateDeckPage {
    pub languages: Vec<LangInfo>,
    pub preselected_language: Option<String>,
}

#[derive(Template)]
#[template(path = "edit_deck.html")]
pub struct EditDeckPage {
    pub deck: DeckInfo,
    pub cards: Vec<CardWithId>,
    pub card_types: Vec<CardType>,
    pub page: i32,
    pub has_next: bool,
    pub type_filter: Option<i32>,
    pub base_url: String,
}

#[derive(Template)]
#[template(path = "cards.html")]
pub struct CardsPage {
    pub cards: Vec<CardSummary>,
    pub page: i32,
    pub has_next: bool,
    pub base_url: String,
    pub title: String,
    pub subtitle: String,
    pub back_url: String,
    pub back_label: String,
}
