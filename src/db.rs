use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::Result;

#[derive(Clone)]
pub struct DB {
    conn: DatabaseConnection,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let url = Self::db_url();
        let conn = Database::connect(&url).await?;

        Migrator::up(&conn, None).await?;

        return Ok(Self { conn });
    }

    fn db_url() -> String {
        let database_user = std::env::var("DATABASE_USER").unwrap_or("backend".to_string());
        let database_password = std::env::var("DATABASE_PASSWORD").unwrap_or("backend".to_string());
        let database_host = std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
        let database_port = std::env::var("DATABASE_PORT").unwrap_or("5454".to_string());
        let database_name = std::env::var("DATABASE_NAME").unwrap_or("backend".to_string());
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database_user, database_password, database_host, database_port, database_name
        );

        return database_url;
    }

    pub fn conn(&self) -> &DatabaseConnection {
        return &self.conn;
    }
}
