# rfd-notify

`rfd-notify` scans the latest posts in the RedFlagDeals Hot Deals forum and sends a notification when a topic title or dealer name matches one of your expressions. Expressions use Python regular expression syntax and are matched without regard to case.

Matches are remembered in a local database so the same topic is only reported once.

## Requirements

- Docker, or [uv](https://docs.astral.sh/uv/) with Python 3.10 or newer
- An [Apprise URL](https://github.com/caronc/apprise#productivity-based-notifications) in the `APPRISE_URL` environment variable

If `APPRISE_URL` is missing, the scan still runs but notifications are skipped.

## Configuration

Create a `config.yml` with an `expressions` list. Each expression is searched in the topic title and, when available, the dealer name.

```yaml
expressions:
  - pizza
  - starbucks¸¸¸¸
  - price error
  - ontario(.*)grocery
```

Patterns are passed to Python's `re` module. Invalid regular expressions will stop the scan.

## Run with Docker

Build the image, then mount your configuration and match database so the database persists across runs:

```sh
docker build -t rfd-notify .
mkdir -p data
docker run --rm \
  -e APPRISE_URL \
  -v "$PWD/config.yml:/app/config.yml:ro" \
  -v "$PWD/data:/app/data" \
  rfd-notify --storage-path /app/data/previous_matches
```

The container expects `/app/config.yml`; the example configuration is available at [`examples/config.yml`](examples/config.yml).

## Run with uv

```sh
uv sync
APPRISE_URL='your-apprise-url' uv run python rfd_notify/cli.py --config examples/config.yml
```

The command defaults to a `previous_matches` database in the current directory. Use `--storage-path` to choose another location:

```sh
uv run python rfd_notify/cli.py --config examples/config.yml --storage-path data/previous_matches
```

## GitHub Actions

Run the action on a schedule and preserve `previous_matches` in the repository between runs. Add an `APPRISE_URL` repository secret and grant the workflow permission to write repository contents.

```yaml
name: rfd-notify

on:
  schedule:
    - cron: "*/5 * * * *"
  workflow_dispatch:

permissions:
  contents: write

jobs:
  notify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run rfd-notify
        uses: davegallant/rfd-notify@main
        env:
          APPRISE_URL: ${{ secrets.APPRISE_URL }}
      - name: Save match history
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
          git add previous_matches
          git diff --cached --quiet || git commit -m "Update previous_matches"
          git push
```

The workflow expects `config.yml` at the repository root. The action reads it and uses `previous_matches` in its working directory. You can use the same steps in Gitea Actions with the corresponding checkout action and secret configuration.

## GitLab Pipelines

Create a [scheduled pipeline](https://docs.gitlab.com/ee/ci/pipelines/schedules.html), configure `APPRISE_URL` as a CI/CD variable, and keep the match database in the job cache:

```yaml
default:
  image:
    name: ghcr.io/davegallant/rfd-notify:2
    entrypoint: [""]

run:
  cache:
    key: previous_matches
    paths:
      - previous_matches
  script:
    - python /app/rfd_notify/cli.py --config config.yml
```

This expects `config.yml` in the repository root.

## Notifications

Notification bodies use Markdown and include a clickable topic title, post age, and the expression that matched. For Gotify, set Markdown formatting in the Apprise URL, for example:

```text
gotifys://gotify.example.com/APP_TOKEN?format=markdown
```
