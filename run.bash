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

# link pepe pictures if POPULATE_DB is set
set -o allexport
source .env
set +o allexport
[ -n "${POPULATE_DB}" ] && ./scripts/link_pepes.bash

# wait for input
echo
echo "Application is running on $FRONT_LINK !"
echo "Press enter, Ctrl+D or Ctrl+C to stop it."
read

# stop application
stop
