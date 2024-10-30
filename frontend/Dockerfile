FROM rust:latest AS builder

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

RUN cargo install wasm-pack
COPY . .

RUN wasm-pack build --dev --target web --out-name wasm --out-dir ./static


FROM rust:latest AS runtime

RUN cargo install simple-http-server

COPY --from=builder /app/static /static

EXPOSE 3000

CMD ["simple-http-server", "-i", "./static/", "-p", "3000", "--nocache", "--try-file", "./static/index.html"]
