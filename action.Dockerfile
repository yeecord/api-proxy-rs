FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY api-proxy-rs /app/api-proxy-rs

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost/ || exit 1

ENTRYPOINT ["/app/api-proxy-rs"]