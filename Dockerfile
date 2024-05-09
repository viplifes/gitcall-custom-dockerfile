FROM rust:1.78-alpine3.19 as builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

FROM alpine:3.19
COPY --from=builder /usr/src/app/target/release/gitcall-rust-app /usr/local/bin/gitcall-rust-app

CMD ["gitcall-rust-app"]