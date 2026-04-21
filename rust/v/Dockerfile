FROM rust:1.77-alpine AS builder

RUN apk add --no-cache musl-dev pkgconf openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs # Optimization trick
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache libgcc

WORKDIR /app
COPY --from=builder /app/target/release/v /app/v

CMD ["./v"
