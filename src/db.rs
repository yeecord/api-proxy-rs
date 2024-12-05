use std::sync::LazyLock;

use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};

pub static DB: LazyLock<Database> = LazyLock::new(Database::default);

pub const SEED_QUERY: &str = r#"CREATE TABLE IF NOT EXISTS cache (
  key INTEGER PRIMARY KEY,
  body BLOB,
  status INTEGER NOT NULL,
  timestamp INTEGER NOT NULL,
  content_type TEXT
)"#;

#[derive(FromRow, Clone)]
pub struct CacheResponse {
  pub key: i64,
  pub body: Option<Vec<u8>>,
  pub status: u16,
  pub content_type: Option<String>,
}

pub struct Database {
  connection: SqlitePool,
}

impl Default for Database {
  fn default() -> Self {
    Self {
      connection: SqlitePool::connect_lazy_with(
        SqliteConnectOptions::new()
          .filename("sqlite.db")
          .create_if_missing(true),
      ),
    }
  }
}

impl Database {
  pub async fn seed(&self) {
    sqlx::query(SEED_QUERY)
      .execute(&self.connection)
      .await
      .unwrap();
  }

  pub async fn get(&self, key: i64) -> Option<CacheResponse> {
    sqlx::query_as(
      r#"SELECT * FROM cache WHERE key = ? AND timestamp > (strftime('%s', 'now') - 60 * 5)"#,
    )
    .bind(key)
    .fetch_optional(&self.connection)
    .await
    .unwrap()
  }

  pub async fn set(&self, response: &CacheResponse) {
    sqlx::query(
      r#"INSERT OR REPLACE INTO cache (key, body, status, timestamp, content_type) VALUES (?, ?, ?, strftime('%s', 'now'), ?)"#,
    ).bind(
      response.key
    ).bind(
      response.body.as_ref(),
    ).bind(
      response.status,
    ).bind(
      response.content_type.as_ref(),
    ).execute(&self.connection)
      .await.unwrap();
  }

  pub async fn delete(&self, key: i64) {
    sqlx::query(r#"DELETE FROM cache WHERE key = ?"#)
      .bind(key)
      .execute(&self.connection)
      .await
      .unwrap();
  }
}
