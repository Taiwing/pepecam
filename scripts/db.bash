#!/usr/bin/env bash

# get the directory name
DIRNAME=$(basename $(git rev-parse --show-toplevel))

# get a postgres prompt
docker exec -it $DIRNAME-db-1 psql -U postgres
