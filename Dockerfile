FROM rust:1-slim-buster AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release -p campaign_server --features=backend

FROM rust:1-slim-buster

COPY --from=builder /code/target/release/campaign_server /usr/bin/campaign_server

EXPOSE 443

ENTRYPOINT [ "/usr/bin/ad_buy_engine" ]
