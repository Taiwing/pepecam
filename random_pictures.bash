#!/usr/bin/env bash

# To use this script you need the application backend to be running. It is to be
# used only once to generate random pictures for the database. Then you have to
# restart the backend and execute the script that will symlink the generated
# pictures to the front/pictures/ directory.

# number of pictures to download
COUNT=1084

function download_pictures() {
	# picsum output directory
	mkdir -p picsum/

	# resolutions
	RESOLUTIONS=(
		"600/900"
		"900/600"
		"1024"
		"1280/900"
		"900/1280"
		"1920/1080"
		"1080/1920"
	)

	# download pictures
	for i in $(seq 0 $COUNT); do
		RESOLUTION=${RESOLUTIONS[ $RANDOM % ${#RESOLUTIONS[@]} ]}
	    wget -O picsum/$i.jpg https://picsum.photos/id/$i/$RESOLUTION
	done
}

function register_user() {
	ID=$(echo $RANDOM | md5sum | head -c 20)
	REGISTER_TOKEN=$(curl -X POST http://localhost:3000/user/register \
		-H 'Content-Type: application/json' \
		--data '{"username":"'$ID'","password":"Trustno1!","email":"'$ID'@b.c"}')
	REGISTER_TOKEN="${REGISTER_TOKEN:10:36}"
	echo $REGISTER_TOKEN
}

function confirm_user() {
	REGISTER_TOKEN=$1
	curl -c jar -X POST http://localhost:3000/user/confirm \
		-H 'Content-Type: application/json' \
		--data '{"token":"'$REGISTER_TOKEN'"}' | jq
}

function generate_pepes() {
	# pepe output directory
	mkdir -p pepe/

	# pepes
	PEPES=($(ls front/pictures/superposables/ | cut -f1 -d'.'))

	# generate pictures
	for i in $(seq 0 $COUNT); do
		PEPE=${PEPES[ $RANDOM % ${#PEPES[@]} ]}
		PICTURE_ID=$(curl -b jar -X POST http://localhost:3000/picture/$PEPE \
			-H 'Content-Type: image/jpeg' \
			--data-binary @picsum/$i.jpg)
		PICTURE_ID="${PICTURE_ID:22:36}"
		mv front/pictures/$PICTURE_ID.jpg pepe/$i-$PEPE.jpg
	done
}

download_pictures
REGISTER_TOKEN=$(register_user)
confirm_user $REGISTER_TOKEN
generate_pepes
