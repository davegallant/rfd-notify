# syntax=docker/dockerfile:1
FROM python:3.12-alpine3.17

WORKDIR /app

COPY . .

RUN pip install --no-cache-dir poetry==1.3.2 && \
  poetry export -f requirements.txt > /tmp/requirements.txt && \
  pip install -r /tmp/requirements.txt

ENTRYPOINT ["python", "/app/rfd_notify/cli.py", "-c", "config.yml"]
