FROM rust:1-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo install cargo-chef && \
  cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN cargo install cargo-chef && \
  cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM scratch AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/api-proxy-rs /app/api-proxy-rs

ENTRYPOINT ["/app/api-proxy-rs"]