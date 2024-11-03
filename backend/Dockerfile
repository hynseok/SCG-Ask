FROM rust:latest as builder

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release


FROM alpine:latest

RUN apk add --no-cache openssl3 libgcc libstdc++ ca-certificates

COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend

RUN chmod +x /usr/local/bin/backend

CMD ["/usr/local/bin/backend"]