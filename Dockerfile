# syntax=docker/dockerfile:1.6

FROM rust:1.93-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --no-default-features \
    && cp target/release/url-shortener /app/url-shortener

FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -g 1000 appuser \
    && useradd -u 1000 -g appuser -s /bin/false appuser \
    && chown -R appuser:appuser /app

USER appuser

COPY --from=builder /app/url-shortener /app/url-shortener
COPY config/template.toml /app/config/template.toml

EXPOSE 3100

CMD ["/app/url-shortener"]
