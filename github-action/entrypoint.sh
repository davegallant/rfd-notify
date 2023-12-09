#!/bin/sh -l

echo "Starting rfd-notify with config: $1"

python "/app/rfd_notify/cli.py" -c "$1"
