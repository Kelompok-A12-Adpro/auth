# Stage 1: Build the Rust binary
FROM rust:1.82 as builder

WORKDIR /app

# Install dependencies for building OpenSSL if needed
RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY . .

# Build with optimizations
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install only runtime dependencies (e.g., OpenSSL)
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/auth /app/main

# Environment variables for your app
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80
EXPOSE 80

CMD ["./main"]
