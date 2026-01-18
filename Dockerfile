FROM python:3.13-slim-trixie AS builder
WORKDIR /app
COPY pyproject.toml poetry.lock* ./
RUN pip install --no-cache-dir poetry==2.2.1 poetry-plugin-export && \
    poetry export -f requirements.txt --without-hashes --output requirements.txt
RUN python -m venv /venv && \
    /venv/bin/pip install --no-cache-dir -r requirements.txt
COPY . .
ENV PATH="/venv/bin:$PATH"
RUN apt-get purge -y --allow-remove-essential bash
ENTRYPOINT ["/venv/bin/python", "rfd_notify/cli.py", "-c", "config.yml"]
