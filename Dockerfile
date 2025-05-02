FROM rust:1.82 AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/auth /usr/local/bin/auth
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/app/diesel.toml /usr/local/bin/

WORKDIR /usr/local/bin

ENV RUST_ENV=main

EXPOSE 80

CMD ["auth"]