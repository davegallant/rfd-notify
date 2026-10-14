FROM python:3.15-slim-trixie AS builder
COPY --from=ghcr.io/astral-sh/uv:0.13.1 /uv /bin/uv
WORKDIR /app
COPY pyproject.toml uv.lock ./
RUN uv export --frozen --no-dev --no-emit-project --no-hashes --format requirements-txt --output-file requirements.txt
RUN python -m venv /venv && \
    /venv/bin/pip install --no-cache-dir -r requirements.txt
COPY . .
ENV PATH="/venv/bin:$PATH"
RUN apt-get purge -y --allow-remove-essential bash
ENTRYPOINT ["/venv/bin/python", "rfd_notify/cli.py", "-c", "config.yml"]
