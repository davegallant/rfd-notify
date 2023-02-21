FROM cgr.dev/chainguard/python:dev-3.11.1 as builder

WORKDIR /app

COPY pyproject.toml .
COPY poetry.lock .
COPY rfd_notify .

RUN pip install poetry==1.3.2 && \
  /home/nonroot/.local/bin/poetry export -f requirements.txt > /tmp/requirements.txt

RUN pip install -r /tmp/requirements.txt --user

FROM cgr.dev/chainguard/python:3.11.1

WORKDIR /app

# Make sure you update Python version in path
COPY --from=builder /home/nonroot/.local/lib/python3.11/site-packages /home/nonroot/.local/lib/python3.11/site-packages

COPY rfd_notify .

ENTRYPOINT ["python", "/app/cli.py", "-c", "config.yml"]
