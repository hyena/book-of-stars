#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[cfg(test)]
extern crate matches;

pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
use diesel::sql_types;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

no_arg_sql_function!(
    RANDOM,
    sql_types::Integer,
    "Represents the SQL RANDOM() function"
);

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set. Check your .env");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_quoth<'a>(
    conn: &SqliteConnection,
    author: i64,
    guild: i64,
    starred_by: i64,
    message_id: i64,
    content: &'a str,
) -> Result<(), DatabaseErrorKind> {
    use schema::quoths;

    let new_quoth = NewQuoth {
        author,
        guild,
        starred_by,
        message_id,
        content,
    };

    match diesel::insert_into(quoths::table)
        .values(&new_quoth)
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(DatabaseError(UniqueViolation, _)) => Err(UniqueViolation),
        Err(error) => panic!("Couldn't insert into database {:?}", error),
    }
}

pub fn random_quoth(conn: &SqliteConnection, author: Option<i64>) -> QueryResult<Quoth> {
    use schema::quoths;

    match author {
        Some(id) => quoths::table
            .filter(quoths::author.eq(id))
            .order(RANDOM)
            .first::<Quoth>(conn),
        None => quoths::table.order(RANDOM).first::<Quoth>(conn),
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    use diesel_migrations::embed_migrations;
    use matches::assert_matches;

    embed_migrations!("migrations");

    fn memory_database_connection() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:").unwrap();
        embedded_migrations::run(&conn).unwrap();
        conn
    }

    #[test]
    fn test_add_quoth() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
    }

    #[test]
    fn test_simple_query() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        add_quoth(&conn, 2, 1, 1, 2, "seize the yeens of production");
        let quoth_1 = random_quoth(&conn, Some(1)).unwrap();
        let quoth_2 = random_quoth(&conn, Some(2)).unwrap();
        assert_eq!(quoth_1.content, "smells like yeen spirit");
        assert_eq!(quoth_2.content, "seize the yeens of production");
    }

    #[test]
    fn test_empty_author() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        let quoth_rand = random_quoth(&conn, None).unwrap();
        assert_eq!(quoth_rand.content, "smells like yeen spirit");
    }

    #[test]
    fn test_missing() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        assert_eq!(
            random_quoth(&conn, Some(3)).err().unwrap(),
            diesel::NotFound
        );
    }

    #[test]
    fn test_duplicate_msg() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        assert_matches!(
            add_quoth(&conn, 2, 1, 2, 1, "seize the yeens"),
            Err(UniqueViolation)
        );
    }
}
