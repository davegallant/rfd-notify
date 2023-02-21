#!/bin/sh -l

echo "Starting rfd-notify with config: $1"

python /app/cli.py -c "$1"
