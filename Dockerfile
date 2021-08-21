FROM rust:1.54-buster@sha256:f91b3dae52c0d99302366331e36cf6af8a923d36c68577df1f4d5a7c2721ca90 as build

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && \
    apt -y install \
    ca-certificates \
    libssl-dev \
    musl-tools \
    musl-dev

RUN rustup target add x86_64-unknown-linux-musl

RUN update-ca-certificates

WORKDIR /usr/src/rfd-notify

COPY Cargo.toml Cargo.toml
COPY src src

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --target=x86_64-unknown-linux-musl --release

RUN mkdir /examples

COPY examples /examples

### Final lightweight image
FROM scratch

COPY --from=build /examples /examples

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/src/rfd-notify/target/x86_64-unknown-linux-musl/release/rfd-notify ./rfd-notify

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/rfd-notify"]
