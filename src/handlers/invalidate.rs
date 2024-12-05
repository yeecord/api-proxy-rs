use axum::Json;

use crate::{
  db::DB,
  hash::{create_cache_key, CacheKeyPayload},
};

pub async fn invalidate_handler(body: Json<CacheKeyPayload>) {
  let key = create_cache_key(body.0);

  DB.delete(key).await;
}
