use askama::Template;
use super::types::LangInfo;

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
