# rfd-notify

![](https://github.com/davegallant/rfd-notify/workflows/ci/badge.svg)
[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/)
[!Docker](https://img.shields.io/docker/pulls/davegallant/rfd-notify?style=plastic)


This tool looks for regular expressions from [RedFlagDeals.com forums](https://forums.redflagdeals.com/hot-deals-f9/) and will send emails based on matches.


## requirements

- a free [SendGrid API key](https://sendgrid.com/pricing/)

## use

```shell
USAGE:
    rfd-notify [OPTIONS] --config <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Specify path to config
    -d, --dbpath <dbpath>    Specify path to where the embedded database is stored [default: ./deals_db]
```

### docker

```shell
# Run the docker image using an example config:
docker run -e RUST_LOG=INFO davegallant/rfd-notify -c /examples/config.toml
```

Provide a custom configuration. An example can found in [config.toml](./examples/config.toml)

```shell
# Provide a custom-config.toml that is in the current directory
# ensuring the correct user is mapped to the working directory
docker run -u "$(id -u):$(id -g)" -w=/tmp -e RUST_LOG=INFO -v "$PWD":/tmp davegallant/rfd-notify -c /tmp/custom-config.toml
```

## cross compile

I had motivations to run this on a Raspberry Pi Zero (without needing docker on the pi):

```shell
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf'
rust-musl-builder cargo build --release
```

The above can be substituted for [other architectures](https://github.com/messense/rust-musl-cross#prebuilt-images).
