FROM ekidd/rust-musl-builder:nightly-2020-04-10 AS builder
WORKDIR /usr/src/app
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/mediaproxy-server /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/mediaproxy-server", "--listen", "0.0.0.0:80"]