# rfd-notify

This tool looks for matches on [RedFlagDeals.com forums](https://forums.redflagdeals.com/hot-deals-f9/) and will send push notifications via [apprise](https://github.com/caronc/apprise).

This was originally written before [alerts](https://www.redflagdeals.com/alerts/) existed. With rfd-notify, alerts never expire and support regular expressions.

## Prerequisites

- either [poetry](https://github.com/python-poetry/poetry), docker, or github actions
- [sendgrid](https://sendgrid.com/pricing/) or gmail can be used for email notifications ([see docs](https://github.com/caronc/apprise/wiki/Notify_email#using-built-in-email-services))

## Usage

The simplest way to get started is to clone this repo, and run with docker:

```sh
docker run -it -v "$(pwd)/examples/config.yml:/app/config.yml" ghcr.io/davegallant/rfd-notify:1
```

To run the code with [poetry](https://python-poetry.org/), clone this repo and run the following:

```shell
poetry install
poetry run python rfd_notify/cli.py -c examples/config.yml
```

### Environment Variables

The following environment variables are required:

| VARIABLE    | DESCRIPTION                                                                              |
| ----------- | ---------------------------------------------------------------------------------------- |
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

## Github Actions (and Gitea Actions)

> Commiting the pickled data (previous_matches) back into the repository is a bit of a hack, but allows for a simpler setup.

An action can be setup to scan for deals, send a notification and store previously found deals in the repo.

It also requires the corresponding [encrypted secrets](https://docs.github.com/en/free-pro-team@latest/actions/reference/encrypted-secrets) setup.

```yaml
# .github/workflows/rfd-notify.yml

on:
  schedule:
    - cron: "*/5 * * * *"
jobs:
  rfd_notify:
    name: rfd-notify
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          sparse-checkout: |
            config.yml
            previous_matches
      - name: Commit and push changes
        run: |
          git config --local user.email "actions@github.com"
          git config --local user.name "RFD Notify"
          git add previous_matches
          git commit -m "Update previous_matches" -a || true
          git push
```

## Gitlab Pipelines

> Create a [scheduled pipeline](https://docs.gitlab.com/ee/ci/pipelines/schedules.html) to run on a regular interval.

With Gitlab Pipelines, the following configuration works:

```yaml
#.gitlab-ci.yml
default:
  image:
    name: ghcr.io/davegallant/rfd-notify:1
    entrypoint: [""]

run:
  cache:
    - key: previous_matches
      paths:
        - previous_matches
  script:
    - python /app/rfd_notify/cli.py -c config.yml
```
