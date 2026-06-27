FROM rust:slim-bookworm AS builder

WORKDIR /code

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        libpq-dev \
        libsqlite3-dev \
        libssl-dev \
        pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY . /code

RUN cargo build -p campaign_server --features=backend --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        libpq5 \
        libsqlite3-0 \
        openssl && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir /app/migrations /app/static

COPY --from=builder /code/target/release/campaign_server /app/campaign_server
COPY migrations /app/migrations
COPY static /app/static

EXPOSE 80
EXPOSE 443

ENTRYPOINT ["/app/campaign_server"]
