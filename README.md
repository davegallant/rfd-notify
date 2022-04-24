# rfd-notify

[![Publish Container](https://github.com/davegallant/rfd-notify/actions/workflows/docker.yml/badge.svg)](https://github.com/davegallant/rfd-notify/actions/workflows/docker.yml)
[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/)

This tool looks for regular expressions from [RedFlagDeals.com forums](https://forums.redflagdeals.com/hot-deals-f9/) and will send emails based on matches.


## Prerequisites

- a free [SendGrid API key](https://sendgrid.com/pricing/)

## Usage

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

### Environment Variables

The following environment variables are required:

| VARIABLE            | DESCRIPTION                                                      |
| ------------------- | ---------------------------------------------------------------- |
| SENDGRID_API_KEY    | Key can be created at https://app.sendgrid.com/settings/api_keys |
| SENDGRID_MAIL_FROM  | The email address that the email is sent from                    |
| SENDGRID_MAIL_TO    | The email address to notify                                      |

## Example Configuration

The following configuration can be passed to `rfd-notify`:

```yaml
# config.yml
expressions:
 - "rx.?5[6789]0"
 - pizza
 - starbucks
```

## Drone CI

The following works on [Drone CI](https://www.drone.io/):


```yaml
# .drone.yml
---
kind: pipeline
type: docker
name: default

steps:

- name: run rfd-notify
  image: ghcr.io/davegallant/rfd-notify
  environment:
   SENDGRID_API_KEY:
    from_secret: sendgrid_api_key
   SENDGRID_MAIL_FROM: notify@rfd-notify.org
   SENDGRID_MAIL_TO: example@example.com

- name: commit db changes
  image: appleboy/drone-git-push
  settings:
    branch: main
    remote_name: origin
    force: false
    commit: true
```

## Github Action

An action can be setup to scan for deals, send a notification and store previously found deals in the repo.

It also requires the corresponding [encrypted secrets](https://docs.github.com/en/free-pro-team@latest/actions/reference/encrypted-secrets) setup.

```yaml
# .github/workflows/main.yml

on:
 schedule:
  - cron: '*5 * * * *'
jobs:
  rfd_notify:
    runs-on: ubuntu-latest
    name: rfd-notify
    steps:
      - name: Checkout
        uses: actions/checkout@v2
        with:
          fetch-depth: 0 # speed boost

      - name: Run rfd-notify
        uses: davegallant/rfd-notify@main
        env:
          SENDGRID_API_KEY: ${{ secrets.SENDGRID_API_KEY }}
          SENDGRID_MAIL_FROM: notify@rfd-notify.org
          SENDGRID_MAIL_TO: ${{ secrets.SENDGRID_MAIL_TO }}

      - name: Commit files
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "RFD Notify"
          git add deals_db/
          git commit -m "Add changes" -a || true

      - name: Push changes
        uses: ad-m/github-push-action@master
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          branch: ${{ github.ref }}
```
