FROM rust@sha256:b2fd2fcc9d28c1a6dc59c3b0b37913fd9a326c8e457e50617e1156fc1ad51e34 as build

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
