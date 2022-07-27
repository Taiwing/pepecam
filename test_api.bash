#!/bin/bash

echo get pictures
curl -X GET http://localhost:3000/pictures
echo

echo register user
REGISTER_TOKEN=$(curl -X POST http://localhost:3000/user/register \
	-H 'Content-Type: application/json' \
	--data '{"username":"BaboucheMan","password":"Trustno1!","email":"a@b.c"}')
REGISTER_TOKEN="${REGISTER_TOKEN:10:36}"
echo REGISTER_TOKEN: $REGISTER_TOKEN
echo

echo confirm user
curl -c jar -X POST http://localhost:3000/user/confirm \
	-H 'Content-Type: application/json' \
	--data '{"token":"'$REGISTER_TOKEN'"}'
