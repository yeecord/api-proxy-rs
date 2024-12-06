use std::sync::LazyLock;

use axum::{
  extract::Request,
  http::{HeaderValue, Uri},
  response::Response,
};
use reqwest::{
  header::{AUTHORIZATION, CONTENT_TYPE, HOST},
  Client,
};

use crate::{
  db::{CacheResponse, DB},
  hash::create_cache_key,
};

pub static HTTP: LazyLock<Client> = LazyLock::new(Client::default);

pub const DISCORD_HOST: HeaderValue = HeaderValue::from_static("discord.com");

pub async fn api_handler(request: Request) -> Response {
  let (mut head, _) = request.into_parts();
  let uri = head.uri.into_parts();

  let Some(path) = uri.path_and_query else {
    return Response::builder()
      .status(400)
      .body("Bad Request".into())
      .unwrap();
  };

  let cache_key = create_cache_key(
    head.method.as_str().as_bytes(),
    path.as_str().as_bytes(),
    head.headers.get(AUTHORIZATION).map(|x| x.as_bytes()),
  );

  if let Some(cache_response) = DB.get(cache_key).await {
    return cache_response.into();
  }

  let url = Uri::builder()
    .authority("discord.com")
    .scheme("https")
    .path_and_query(path)
    .build()
    .unwrap();

  head.headers.insert(HOST, DISCORD_HOST);

  let mut response = HTTP
    .get(url.to_string())
    .headers(head.headers)
    .send()
    .await
    .unwrap();

  let status = response.status().as_u16();

  let content_type = response.headers_mut().remove(CONTENT_TYPE);
  let body = response.bytes().await.unwrap();

  let cache_payload = CacheResponse {
    key: cache_key,
    content_type: content_type.map(|x| x.to_str().unwrap().to_string()),
    body: Some(body.to_vec()),
    status,
  };

  DB.set(&cache_payload).await;

  cache_payload.into()
}
