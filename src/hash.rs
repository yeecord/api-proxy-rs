use serde::Deserialize;
use xxhash_rust::xxh3;

#[derive(Debug, Deserialize)]
pub struct CacheKeyPayload {
  pub method: String,
  pub url: String,
  pub authorization: Option<String>,
}

pub fn create_cache_key(payload: CacheKeyPayload) -> u64 {
  let method_bytes = payload.method.as_bytes();
  let url_bytes = payload.url.as_bytes();
  let authorization_bytes = payload
    .authorization
    .as_deref()
    .unwrap_or_default()
    .as_bytes();

  let mut buffer =
    Vec::with_capacity(method_bytes.len() + url_bytes.len() + authorization_bytes.len());

  buffer.extend_from_slice(method_bytes);
  buffer.extend_from_slice(url_bytes);
  buffer.extend_from_slice(authorization_bytes);

  xxh3::xxh3_64(&buffer)
}
