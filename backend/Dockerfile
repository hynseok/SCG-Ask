FROM rust:latest as builder

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release


FROM debian:bullseye-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    openssl \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

CMD ["backend"]