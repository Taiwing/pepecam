#!/usr/bin/env bash

# stop application and exit
function stop() {
    docker compose down
    exit 0
}

# go to repo root
cd $(git rev-parse --show-toplevel)

# build and run docker compose
docker compose up --build --wait

# set SIGINT handler
trap 'stop' SIGINT

# link pepe pictures
./scripts/link_pepes.bash

# wait for input
read

# stop application
stop
