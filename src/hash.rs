use serde::Deserialize;
use xxhash_rust::xxh3;

#[derive(Debug, Deserialize)]
pub struct CacheKeyPayload {
  pub method: String,
  pub url: String,
  pub host: String,
  pub authorization_header: String,
  pub authorization: Option<String>,
}

pub fn create_cache_key(
  method: &[u8],
  url: &[u8],
  host: &[u8],
  authorization_header: &[u8],
  authorization: Option<&[u8]>,
) -> i64 {
  let mut buffer = Vec::with_capacity(
    method.len()
      + url.len()
      + host.len()
      + authorization_header.len()
      + authorization.map_or(0, |x| x.len()),
  );

  buffer.extend_from_slice(method);
  buffer.extend_from_slice(url);
  buffer.extend_from_slice(host);
  buffer.extend_from_slice(authorization_header);

  if let Some(authorization) = authorization {
    buffer.extend_from_slice(authorization);
  }

  xxh3::xxh3_64(&buffer) as i64
}
