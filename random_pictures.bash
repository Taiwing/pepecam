#!/usr/bin/env bash

# To use this script you need the application backend to be running. It is to be
# used only once to generate random pictures for the database. Then you have to
# restart the backend and execute the script that will symlink the generated
# pictures to the front/pictures/ directory.

# number of pictures to download
COUNT=1084

# picture resolutions
RESOLUTIONS=(
	"600/900"
	"900/600"
	"1024/1024"
	"1280/900"
	"900/1280"
	"1920/1080"
	"1080/1920"
)

function download_pictures() {
	# picsum output directory
	mkdir -p picsum/

	# download pictures
	for i in $(seq 0 $COUNT); do
		RESOLUTION=${RESOLUTIONS[ $RANDOM % ${#RESOLUTIONS[@]} ]}
	    wget -O picsum/$i.jpg https://picsum.photos/id/$i/$RESOLUTION
	done
}

function login_user() {
	curl -c jar -X POST http://localhost:3000/user/login \
		-H 'Content-Type: application/json' \
		--data '{"username":"User1","password":"Trustno1!"}' | jq
}

function generate_pepes() {
	# pepe output directory
	mkdir -p pepe/

	# pepes
	PEPES=($(ls front/pictures/superposables/ | cut -f1 -d'.'))

	# generate pictures
	for i in $(seq 0 $COUNT); do
		[ ! -s "picsum/$i.jpg" ] && continue
		PEPE=${PEPES[ $RANDOM % ${#PEPES[@]} ]}
		PICTURE_ID=$(curl -b jar -X POST http://localhost:3000/picture/$PEPE \
			-H 'Content-Type: image/jpeg' \
			--data-binary @picsum/$i.jpg)
		PICTURE_ID="${PICTURE_ID:22:36}"
		mv front/pictures/$PICTURE_ID.jpg pepe/$i-$PEPE.jpg
	done

	# move pictures to front/pictures/
	mkdir -p front/pictures/pepe
	mv pepe/* front/pictures/pepe/
}

# use this instead of download_pictures() if some pictures are missing
function download_missing() {
	# download pictures
	for i in $(seq 0 $COUNT); do
		if [ ! -s "picsum/$i.jpg" ]; then
			RESOLUTION=${RESOLUTIONS[ $RANDOM % ${#RESOLUTIONS[@]} ]}
			wget -O picsum/$i.jpg https://loremflickr.com/$RESOLUTION
		fi
	done
}

download_pictures
login_user
generate_pepes
