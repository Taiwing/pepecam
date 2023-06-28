#!/usr/bin/env bash

# go to repo root
cd $(git rev-parse --show-toplevel)

# build and run docker compose
docker compose up --build --wait

# link pepe pictures
./scripts/link_pepes.bash

# wait for input
read

# stop application
docker compose down
