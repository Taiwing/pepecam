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
./test_api.bash

# replace default password_hash
PASSWORD_HASH=$(
	docker exec camagru-db-1 psql -U postgres postgres -t -c \
	"SELECT password_hash FROM accounts WHERE password_hash != 'toto' LIMIT 1;" \
	| tr -d '[:space:]'
)
docker exec camagru-db-1 psql -U postgres postgres -c \
	"UPDATE accounts SET password_hash = '$PASSWORD_HASH' WHERE password_hash = 'toto';"

# wait for input
read

# stop application
kill -9 $(pidof target/debug/api)
docker compose down
