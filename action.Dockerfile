FROM alpine AS runtime

WORKDIR /app

COPY api-proxy-rs /app/api-proxy-rs

ENTRYPOINT ["/app/api-proxy-rs"]