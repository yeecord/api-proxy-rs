use axum::Json;

use crate::{
  db::DB,
  hash::{create_cache_key, CacheKeyPayload},
};

pub async fn invalidate_handler(body: Json<CacheKeyPayload>) {
  let key = create_cache_key(
    body.method.as_bytes(),
    body.url.as_bytes(),
    body.authorization.as_ref().map(|header| header.as_bytes()),
  );

  DB.delete(key).await;
}
