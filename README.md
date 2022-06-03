# camagru

Snapchat-like web application. Users can login, take pictures through their
webcam, upload pictures and add custom items on them. They can comment and like
each other's pictures and delete their own posts. This is meant to be usable on
desktop and on phones and small resolutions.

## Setup

```shell
# clone it
git clone https://github.com/Taiwing/camagru
# build
docker-compose up
```
Click [here](http://localhost:8080) to test it.

## How it works

This application is bundled with docker-compose. Each part is a service in the
docker-compose.yml file at the root of the repository.

### api

This is the backend api which handles every user request regarding their session
or data. It is a simple HTTP server implemented in Rust with the Rocket library.
It can be accessed on `localhost:3000`.

### db

A Postgresql database storing all the website user related data.

### front

This is the user interface. A simple apache server running on `localhost:8080`
and serving html/CSS/Javascript files.

## Development

Use the dev.bash startup script to build in dev mode.

## TODO

When this app is done, remove or rename api/Dockerfile and use
api/Dockerfile.release instead.
