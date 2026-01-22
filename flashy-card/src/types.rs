use sqlx::FromRow;

/// Overview of information on a language for list views.
#[derive(FromRow, Debug)]
pub struct LangInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}
