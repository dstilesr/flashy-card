use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::languages)]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::languages)]
pub struct NewLanguage<'a> {
    pub name: &'a str,
}
