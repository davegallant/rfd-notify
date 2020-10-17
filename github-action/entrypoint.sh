#!/bin/sh -l

echo "Starting rfd-notify with config: $1"

rfd-notify -c "$1"
