FROM rust@sha256:f91b3dae52c0d99302366331e36cf6af8a923d36c68577df1f4d5a7c2721ca90 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get -y install \
    ca-certificates \
    libssl-dev \
    musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/rfd-notify

COPY Cargo.toml Cargo.toml
COPY src src

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN mkdir /examples
COPY examples /examples

FROM scratch

COPY --from=build /examples /examples

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/src/rfd-notify/target/x86_64-unknown-linux-musl/release/rfd-notify .

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/rfd-notify"]
