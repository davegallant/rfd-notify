FROM rust:1.60.0-buster as build

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

### Final lightweight image
FROM scratch

COPY --from=build /examples /examples

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/src/rfd-notify/target/x86_64-unknown-linux-musl/release/rfd-notify ./rfd-notify

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/rfd-notify"]
