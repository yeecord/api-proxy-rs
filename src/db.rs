use axum::response::Response;
use reqwest::header::CONTENT_TYPE;
use sqlx::Statement;
use std::sync::LazyLock;

use sqlx::{
  sqlite::{SqliteConnectOptions, SqliteStatement},
  Executor, FromRow, SqlitePool,
};
use tokio::sync::OnceCell;

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

impl From<CacheResponse> for Response {
  fn from(val: CacheResponse) -> Self {
    let mut builder = Response::builder().status(val.status);

    if let Some(content_type) = val.content_type {
      builder = builder.header(CONTENT_TYPE, content_type);
    }

    builder.body(val.body.unwrap_or_default().into()).unwrap()
  }
}

pub struct Database<'a> {
  connection: SqlitePool,
  queries: OnceCell<Queries<'a>>,
}

impl Default for Database<'_> {
  fn default() -> Self {
    Self {
      connection: SqlitePool::connect_lazy_with(
        SqliteConnectOptions::new()
          .filename("sqlite.db")
          .create_if_missing(true),
      ),
      queries: OnceCell::default(),
    }
  }
}

impl<'a> Database<'a> {
  pub async fn seed(&self) {
    sqlx::query(SEED_QUERY)
      .execute(&self.connection)
      .await
      .unwrap();

    self
      .queries
      .set(Queries::create(&self.connection).await)
      .unwrap();
  }

  pub async fn get(&self, key: i64) -> Option<CacheResponse> {
    let select_query = &self.queries.get().unwrap().select;

    select_query
      .query_as()
      .bind(key)
      .fetch_optional(&self.connection)
      .await
      .unwrap()
  }

  pub async fn set(&self, response: &CacheResponse) {
    let insert_query = &self.queries.get().unwrap().insert;

    insert_query
      .query()
      .bind(response.key)
      .bind(response.body.as_ref())
      .bind(response.status)
      .bind(response.content_type.as_ref())
      .execute(&self.connection)
      .await
      .unwrap();
  }

  pub async fn delete(&self, key: i64) {
    let delete_query = &self.queries.get().unwrap().delete;

    delete_query
      .query()
      .bind(key)
      .execute(&self.connection)
      .await
      .unwrap();
  }
}

#[derive(Debug)]
struct Queries<'a> {
  pub insert: SqliteStatement<'a>,
  pub select: SqliteStatement<'a>,
  pub delete: SqliteStatement<'a>,
}

impl Queries<'_> {
  pub async fn create(connection: &SqlitePool) -> Self {
    let insert = connection.prepare(
      r#"INSERT OR REPLACE INTO cache (key, body, status, timestamp, content_type) VALUES (?, ?, ?, strftime('%s', 'now'), ?)"#,
    ).await.unwrap();

    let select = connection
      .prepare(
        r#"SELECT * FROM cache WHERE key = ? AND timestamp > (strftime('%s', 'now') - 60 * 5)"#,
      )
      .await
      .unwrap();

    let delete = connection
      .prepare(r#"DELETE FROM cache WHERE key = ?"#)
      .await
      .unwrap();

    Self {
      insert,
      select,
      delete,
    }
  }
}
