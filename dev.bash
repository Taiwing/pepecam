# build and run docker compose
COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker compose build
docker compose up -d

# wait for db to be ready
sleep 7

# build api (TODO: remove this when api is back in compose file)
cd api
./dev.bash &
cd ..

# link pepe pictures
./link_pepes.bash

# test api
#./test_api.bash

# replace default password_hash
#PASSWORD_HASH=$(
#	docker exec camagru-db-1 psql -U postgres postgres \
#	-c \
#	"SELECT password_hash FROM accounts WHERE password_hash IS NOT 'toto' LIMIT 1;"
#)
#
#echo "PASSWORD_HASH: $PASSWORD_HASH"

# wait for input
read

# stop application
kill -9 $(pidof target/debug/api)
docker compose down
