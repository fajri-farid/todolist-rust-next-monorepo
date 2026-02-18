FROM rust:1.88-slim AS builder

WORKDIR /app

COPY apps/api/Cargo.toml apps/api/Cargo.lock ./
COPY apps/api/src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api /usr/local/bin/api

EXPOSE 8080

CMD ["api"]
