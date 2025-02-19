FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:latest AS runtime
WORKDIR /app

RUN apk upgrade --no-cache && \
  apk add --no-cache curl && \
  rm -rf /var/cache/apk/*

COPY --from=builder /app/target/release/api-proxy-rs /usr/local/bin

ENTRYPOINT ["/usr/local/bin/api-proxy-rs"]