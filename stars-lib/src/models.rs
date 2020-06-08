use super::schema::quoths;

/// Some fields are Options (nullable) for historic reasons
#[derive(Queryable)]
#[derive(Debug)]
pub struct Quoth {
    pub id: i64,
    pub author: Option<i64>,
    pub guild: Option<i64>,
    pub starred_by: Option<i64>,
    pub message_id: Option<i64>,
    pub content: String,
    pub legacy: bool,
    pub legacy_author_fallback: Option<String>,
}

/// Standard struct for a new post coming from discord.
#[derive(Insertable)]
#[table_name = "quoths"]
pub struct NewQuoth<'a> {
    pub author: i64,
    pub guild: i64,
    pub starred_by: i64,
    pub message_id: i64,
    pub content: &'a str,
}

/// For inserting old messages from before the jump to discord.
#[derive(Insertable)]
#[table_name = "quoths"]
pub struct NewLegacyQuoth {
    pub author: Option<i64>,
    pub legacy_author_fallback: Option<String>,
    pub content: String,
}
