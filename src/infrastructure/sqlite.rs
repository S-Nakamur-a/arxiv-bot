use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use super::diesel::connection::SimpleConnection;


pub struct SQLite<'a> {
    database: &'a str,
}

impl SQLite<'_> {
    pub fn create() -> SQLite<'static> {
        SQLite {
            database: "database/arxiv-bot.db",
        }
    }

    pub fn connect(self) -> SqliteConnection {
        SqliteConnection::establish(self.database).unwrap()
    }

    pub fn query(self, connection: Option<SqliteConnection>, query: &str) -> QueryResult<()> {
        connection.unwrap_or(self.connect()).batch_execute(query)
    }
}
