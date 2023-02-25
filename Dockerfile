# syntax=docker/dockerfile:1
FROM python:3.11-slim-buster

WORKDIR /app

COPY . .

RUN pip install --no-cache-dir poetry==1.3.2 && \
  poetry install --no-root

ENTRYPOINT ["poetry", "run", "-C", "/app", "python", "/app/rfd_notify/cli.py", "-c", "config.yml"]
