FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install ca-certificates -y && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

COPY api-proxy-rs /usr/local/bin/api-proxy-rs

WORKDIR /app

ENTRYPOINT ["/usr/local/bin/api-proxy-rs"]