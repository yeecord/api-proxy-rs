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
  hash::{create_cache_key, CacheKeyPayload},
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

  let cache_key = create_cache_key(CacheKeyPayload {
    method: head.method.to_string(),
    url: path.to_string(),
    authorization: head
      .headers
      .get(AUTHORIZATION)
      .map(|x| x.to_str().unwrap().to_string()),
  });

  if let Some(cache_response) = DB.get(cache_key).await {
    return response_from_cache(cache_response);
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

  let status = response.status();

  let content_type = response.headers_mut().remove(CONTENT_TYPE);
  let body = response.bytes().await.unwrap();

  let cache_payload = CacheResponse {
    key: cache_key,
    content_type: content_type.map(|x| x.to_str().unwrap().to_string()),
    body: Some(body.to_vec()),
    status: status.as_u16(),
  };

  DB.set(&cache_payload).await;

  response_from_cache(cache_payload)
}

fn response_from_cache(cache_response: CacheResponse) -> Response {
  let mut builder = Response::builder().status(cache_response.status);

  if let Some(content_type) = &cache_response.content_type {
    builder = builder.header(CONTENT_TYPE, content_type);
  }

  builder
    .body(cache_response.body.unwrap_or_default().into())
    .unwrap()
}
