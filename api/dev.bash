#!/usr/bin/env bash

# TODO: Only used for dev purposes, delete this when done
export API_PORT=3000
export RUST_BACKTRACE=1
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=${API_PORT}
export ROCKET_SECRET_KEY=17tu+oEIpQy3OwTKhEDUIhr1HFWyKcb92xzPpLS3t3Y=
export ROCKET_DATABASES="{postgres={url=postgres://postgres:Trustno1@localhost:5432/postgres}}"

cargo run
