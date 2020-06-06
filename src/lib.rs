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

use diesel::prelude::*;
#[cfg(test)]
use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::NotFound, Error::DatabaseError};
use diesel::sql_types;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::*;
use schema::quoths;

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
) -> QueryResult<usize> {
    let new_quoth = NewQuoth {
        author,
        guild,
        starred_by,
        message_id,
        content,
    };

    diesel::insert_into(quoths::table)
        .values(&new_quoth)
        .execute(conn)
}

pub fn random_quoth(conn: &SqliteConnection, author: Option<i64>) -> QueryResult<Quoth> {
    match author {
        Some(id) => quoths::table
            .filter(quoths::author.eq(id))
            .order(RANDOM)
            .first::<Quoth>(conn),
        None => quoths::table.order(RANDOM).first::<Quoth>(conn),
    }
}

pub fn delete_quoth(conn: &SqliteConnection, id: i64) -> QueryResult<usize> {
    diesel::delete(quoths::table.filter(quoths::id.eq(id))).execute(conn)
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
        assert_matches!(
            random_quoth(&conn, Some(3)),
            Err(diesel::NotFound)
        );
    }

    #[test]
    fn test_duplicate_msg() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        assert_matches!(
            add_quoth(&conn, 2, 1, 2, 1, "seize the yeens"),
            Err(DatabaseError(UniqueViolation, _))
        );
    }

    #[test]
    fn test_delete_empty() {
        let conn = memory_database_connection();
        assert_eq!(delete_quoth(&conn, 1).unwrap(), 0);
    }

    #[test]
    fn test_deletes() {
        let conn = memory_database_connection();
        add_quoth(&conn, 1, 1, 2, 1, "smells like yeen spirit");
        add_quoth(&conn, 2, 1, 1, 2, "seize the yeens of production");

        delete_quoth(&conn, 1);
        let quoth_2 = random_quoth(&conn, Some(2)).unwrap();
        assert_eq!(quoth_2.author, Some(2));
        assert_eq!(quoth_2.content, "seize the yeens of production");
        assert_matches!(random_quoth(&conn, Some(1)), Err(NotFound));

        delete_quoth(&conn, 2);
        assert_matches!(random_quoth(&conn, Some(2)), Err(NotFound));
    }
}
