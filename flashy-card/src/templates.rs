use askama::Template;
use super::types::{LangInfo, DeckSummary, CardType};

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
