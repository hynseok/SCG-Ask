FROM rust:latest AS builder

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

EXPOSE 8000

CMD ["cargo", "run", "--release"]