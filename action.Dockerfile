FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && \
  apt-get install -y curl && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY api-proxy-rs /app/api-proxy-rs

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost/ || exit 1

ENTRYPOINT ["/app/api-proxy-rs"]