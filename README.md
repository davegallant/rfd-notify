# rfd-notify

![](https://github.com/davegallant/rfd-notify/workflows/ci/badge.svg)
[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/)
![Docker](https://img.shields.io/docker/pulls/davegallant/rfd-notify?style=plastic)


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

Provide a custom configuration. An example can found in [config.yaml](./examples/config.yaml).

```shell
# Provide a custom-config.yaml that is in the current directory
# ensuring the correct user is mapped to the working directory
docker run -u "$(id -u):$(id -g)" -w=/tmp -e RUST_LOG=INFO -v "$PWD":/tmp davegallant/rfd-notify -c /tmp/custom-config.yaml
```

## github action

TBD
