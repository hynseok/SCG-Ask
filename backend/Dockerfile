FROM rust:latest as builder

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release


FROM debian:bullseye-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    wget \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN wget https://www.openssl.org/source/openssl-3.0.0.tar.gz && \
    tar -xzf openssl-3.0.0.tar.gz && \
    cd openssl-3.0.0 && \
    ./config --prefix=/usr/local/ssl --openssldir=/usr/local/ssl && \
    make && \
    make install && \
    cd .. && \
    rm -rf openssl-3.0.0 openssl-3.0.0.tar.gz

ENV LD_LIBRARY_PATH="/usr/local/ssl/lib"

COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

CMD ["backend"]