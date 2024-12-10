# api-proxy-rs

A simple SQLite caching proxy written in Rust.

## Usage

### GET `/*`

Everything would have a caching ttl for 300 seconds.

#### Headers

- `x-host`: The host to proxy to. Defaults to `discord.com`.
- `x-authorization-name`: The name of the authorization header. Defaults to `authorization`.
- `authorization (or value from x-authorization-name)`: The authorization header value.

### DELETE `/*`

Deletes the cache for the given path.

You should provide the same headers as the GET request.

### GET `/`

Health check endpoint.


## Quick Start with Docker

```sh
docker run -d -p 3000:80 ghcr.io/yeecord/api-proxy-rs:latest
```

## Compile from source

```sh
cargo build --release
```
