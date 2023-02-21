# rfd-notify

[![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/)

This tool looks for matches on [RedFlagDeals.com forums](https://forums.redflagdeals.com/hot-deals-f9/) and supports [apprise](https://github.com/caronc/apprise) push notifications.

This was originally written before [alerts](https://www.redflagdeals.com/alerts/) existed. With rfd-notify, alerts never expire and support regular expressions.

## Prerequisites

- either [poetry](https://github.com/python-poetry/poetry), docker, or github actions
- a free [SendGrid API key](https://sendgrid.com/pricing/) is suggested for email notifications

## Usage

To run the code directly, clone this repo and run the following:

```shell
poetry install
poetry run python rfd_notify/cli.py -c examples/config.yml
```

### Environment Variables

The following environment variables are required:

| VARIABLE    | DESCRIPTION                                                      |
| ----------- | ---------------------------------------------------------------- |
| APPRISE_URL | See [notifications](https://github.com/caronc/apprise#productivity-based-notifications). |

## Example Configuration

Pass a list of expressions to be used for discovering deals:

```yaml
# config.yml
expressions:
  - pizza
  - starbucks
  - price error
```

## Github Action

An action can be setup to scan for deals, send a notification and store previously found deals in the repo.

It also requires the corresponding [encrypted secrets](https://docs.github.com/en/free-pro-team@latest/actions/reference/encrypted-secrets) setup.

```yaml
# .github/workflows/rfd-notify.yml

on:
  schedule:
    - cron: "*5 * * * *"
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
          APPRISE_URL: ${{ secrets.APPRISE_URL }}

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
      APPRISE_URL:
        from_secret: apprise_url

  - name: commit db changes
    image: appleboy/drone-git-push:0.2.1
    settings:
      branch: main
      remote_name: origin
      force: false
      commit: true
```
