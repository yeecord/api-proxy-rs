FROM alpine AS runtime

COPY api-proxy-rs /usr/local/bin/api-proxy-rs

WORKDIR /app

ENTRYPOINT ["/usr/local/bin/api-proxy-rs"]