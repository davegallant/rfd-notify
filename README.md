# rfd-notify

## requirements

- a free [SendGrid API key](https://sendgrid.com/pricing/)

## use

Declare a configuration in [config.toml](./examples/config.toml)

```shell
rfd-notify ./examples/config.toml
```

## cross compile

I had motivations to run this on a Raspberry Pi Zero:

```shell
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf'
rust-musl-builder cargo build --release
```

The above can be substituted for [other architectures](https://github.com/messense/rust-musl-cross#prebuilt-images).
