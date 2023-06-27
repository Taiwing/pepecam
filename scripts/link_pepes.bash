#!/usr/bin/env bash

# go to repo root
cd $(git rev-parse --show-toplevel)

# Get every picture id from the database
DB_PICTURES=$(docker exec camagru-db-1 psql -U postgres postgres \
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
[ ${#MISSING_PICTURE[@]} -eq 0 ] && exit 0

# shuffle the array
MISSING_PICTURE=($(shuf -e "${MISSING_PICTURE[@]}"))

# symlinking missing pictures
INDEX=0
for picture in ${MISSING_PICTURE[@]}; do
	[ ! -f pepe/$INDEX-*.jpg ] && continue
	ln -s pepe/$INDEX-*.jpg $picture.jpg
	INDEX=$((INDEX + 1))
done
