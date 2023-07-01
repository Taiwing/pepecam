#!/usr/bin/env bash

# get the script name
SCRIPT=$(basename $0)

# go to repo root
cd $(git rev-parse --show-toplevel)

# Get every picture id from the database
DB_PICTURES=$(docker exec pepecam-db-1 psql -U postgres postgres \
	-c "SELECT picture_id FROM pictures" \
	| grep -E '[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}')

cd front/pictures/

# Remove old symlinks
find ./ -maxdepth 1 -type l -delete

# Get the subset of picture id that do not match a file in front/pictures/
MISSING_PICTURE=()
for picture in $DB_PICTURES; do
	if [ ! -f $picture.jpg ]; then
		MISSING_PICTURE+=($picture)
	fi
done

# if no picture is missing, exit
if [ ${#MISSING_PICTURE[@]} -eq 0 ]; then
	echo "$SCRIPT: No picture missing."
	exit 0
fi

# shuffle the array
MISSING_PICTURE=($(shuf -e "${MISSING_PICTURE[@]}"))

# symlinking missing pictures
INDEX=0
LINKED=()
SUPERPOSABLES=()
for picture in ${MISSING_PICTURE[@]}; do
	[ ! -f pepe/$INDEX-*.jpg ] && continue
	ln -s pepe/$INDEX-*.jpg $picture.jpg
	SUPERPOSABLE=$(echo pepe/$INDEX-*.jpg | sed -n "s/pepe\/$INDEX-\(.*\)\.jpg/\1/p")
	INDEX=$((INDEX + 1))
	LINKED+=($picture)
	SUPERPOSABLES+=($SUPERPOSABLE)
done

## update superposable column in database
QUERY=""
for i in "${!LINKED[@]}"; do
	QUERY+="UPDATE pictures SET superposable = '${SUPERPOSABLES[$i]}'::superposable WHERE picture_id = '${LINKED[$i]}'::uuid;"
done
docker exec pepecam-db-1 psql -U postgres postgres -c "$QUERY"

echo "$SCRIPT: $INDEX pictures linked."
