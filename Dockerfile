# Build stage
FROM rust:slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Run stage
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/coolify-expo-notification-relay /app/coolify-expo-notification-relay

EXPOSE 3000

CMD ["./coolify-expo-notification-relay"]
