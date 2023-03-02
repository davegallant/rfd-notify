# rfd-notify

This tool looks for matches on [RedFlagDeals.com forums](https://forums.redflagdeals.com/hot-deals-f9/) and will send push notifications via [apprise](https://github.com/caronc/apprise).

This was originally written before [alerts](https://www.redflagdeals.com/alerts/) existed. With rfd-notify, alerts never expire and support regular expressions.

## Prerequisites

- either [poetry](https://github.com/python-poetry/poetry), docker, or github actions
- a free [SendGrid API key](https://sendgrid.com/pricing/) is suggested for email notifications

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

> Commiting the pickled data (previous_matches) back into the repository is a bit of a hack, but allows for a simpler setup.

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
        uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - name: Run rfd-notify
        uses: davegallant/rfd-notify@main
        env:
          APPRISE_URL: ${{ secrets.APPRISE_URL }}

      - name: Commit files
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "RFD Notify"
          git add previous_matches
          git commit -m "Update previous_matches" -a || true

      - name: Push changes
        uses: ad-m/github-push-action@master
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          branch: ${{ github.ref }}
```

## Jenkins

> The necessary Jenkins plugins (such as docker) and credentials must be configured.

Using a declarative pipeline, run the build every minute, and store the previous matches in the workspace:

```groovy
pipeline {
    agent any

    triggers {
        cron('* * * * *')
    }

    options {
        buildDiscarder(
            logRotator(
                numToKeepStr: '25',
                artifactNumToKeepStr: '25'
            )
        )
        disableConcurrentBuilds()
    }

    stages {
        stage('Run rfd-notify') {
            agent {
                docker {
                    image 'ghcr.io/davegallant/rfd-notify:1'
                    args '--entrypoint='
                    reuseNode true
                }
            }
            steps {
                withCredentials([string(credentialsId: 'apprise-url', variable: 'APPRISE_URL')]) {
                    sh 'python /app/rfd_notify/cli.py -c config.yml'
                }
            }
        }
        stage('Archive previous_matches') {
            steps {
                archiveArtifacts artifacts: 'previous_matches'
            }
        }
    }
}
```
