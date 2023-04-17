#!/bin/bash

echo get pictures
curl -X GET 'http://localhost:3000/pictures?index=0&count=10' \
	-H 'Content-Type: application/json'
echo

echo register user
ID=$(echo $RANDOM | md5sum | head -c 20)
echo USERNAME: $ID
REGISTER_TOKEN=$(curl -X POST http://localhost:3000/user/register \
	-H 'Content-Type: application/json' \
	--data '{"username":"'$ID'","password":"Trustno1!","email":"'$ID'@b.c"}')
REGISTER_TOKEN="${REGISTER_TOKEN:10:36}"
echo REGISTER_TOKEN: $REGISTER_TOKEN
echo

echo confirm user
curl -c jar -X POST http://localhost:3000/user/confirm \
	-H 'Content-Type: application/json' \
	--data '{"token":"'$REGISTER_TOKEN'"}' | jq
