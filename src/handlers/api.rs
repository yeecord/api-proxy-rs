use std::sync::LazyLock;

use axum::{
  extract::Request,
  http::{Method, Uri},
  response::Response,
};
use reqwest::{
  header::{CONTENT_TYPE, HOST},
  Client,
};

use crate::{
  db::{CacheResponse, DB},
  hash::create_cache_key,
};

pub static HTTP: LazyLock<Client> = LazyLock::new(Client::default);

pub async fn api_handler(request: Request) -> Response {
  if request.method() != Method::GET && request.method() != Method::DELETE {
    return Response::builder()
      .status(405)
      .body("Method Not Allowed".into())
      .unwrap();
  }

  let (mut head, _) = request.into_parts();
  let uri = head.uri.into_parts();

  let Some(path) = uri.path_and_query else {
    return Response::builder()
      .status(400)
      .body("Bad Request".into())
      .unwrap();
  };

  let host = head
    .headers
    .get("x-host")
    .and_then(|host| host.to_str().ok())
    .unwrap_or("discord.com")
    .to_lowercase();

  let authorization_header = head
    .headers
    .get("x-authorization-name")
    .map(|x| x.to_str().unwrap())
    .unwrap_or("authorization")
    .to_lowercase();

  let authorization = head
    .headers
    .get(&authorization_header)
    .and_then(|x| x.to_str().ok());

  let cache_key = create_cache_key(
    head.method.as_str().as_bytes(),
    path.as_str().as_bytes(),
    host.as_bytes(),
    authorization_header.as_bytes(),
    authorization.map(|x| x.as_bytes()),
  );

  if head.method == Method::DELETE {
    DB.delete(cache_key).await;

    return Response::builder().status(200).body("OK".into()).unwrap();
  }

  if let Some(cache_response) = DB.get(cache_key).await {
    return cache_response.into();
  }

  let url = Uri::builder()
    .authority(host.as_str())
    .scheme("https")
    .path_and_query(path)
    .build()
    .unwrap();

  head.headers.insert(HOST, host.parse().unwrap());

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
