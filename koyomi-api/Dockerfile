# Build Stage
FROM rust:1.81.0-alpine3.20 AS builder

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app
COPY ../Cargo.toml ../Cargo.lock ./
COPY koyomi-core/ ./koyomi-core
COPY  .sqlx/ .

RUN USER=root cargo new koyomi-scraper
RUN USER=root cargo new koyomi-api

WORKDIR /app/koyomi-api
COPY ./koyomi-api/Cargo.toml .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release -p koyomi-api

COPY ./koyomi-api/src/ ./src
COPY ../.sqlx ../.sqlx

RUN cargo build --release -p koyomi-api

# Final State
FROM alpine:latest
COPY --from=builder /app/target/release/koyomi-api .

RUN apk upgrade --no-cache && \
    apk add --no-cache libgcc

EXPOSE 3000

USER 1000
CMD ["./koyomi-api"]

