# Build stage
FROM rust:1.82 as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev

COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/auth ./main

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

EXPOSE 80
CMD ["./main"]
