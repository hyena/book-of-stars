use super::schema::quoths;

#[derive(Queryable)]
#[derive(Insertable)]
pub struct Quoth {
    pub id: i64,
    pub author: Option<i64>,
    pub guild: Option<i64>,
    pub starred_by: Option<i64>,
    pub content: String,
    pub legacy: bool,
    pub legacy_author_fallback: Option<String>
}
