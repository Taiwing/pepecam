#!/usr/bin/env bash

# load .env file
set -o allexport
source ../.env
source ../.env.secrets
set +o allexport

# Use localhost instead of db for the host
export ROCKET_DATABASES="{${DB_USER}={url=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_USER}}}"

rustfmt src/*.rs
cargo run
