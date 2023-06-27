# Pepecam

Snapchat-like web application corresponding to 42's Camagru project. Users can
login, take pictures through their webcam, upload pictures and add custom items
on them. They can comment and like each other's pictures and delete their own
posts. This is meant to be usable on desktop, phones and small resolutions.

<br />
<p align="center">
	<img src="https://github.com/Taiwing/ft_nm/blob/master/front/pictures/superposables/smirk.png?raw=true" alt="Smirking Pepe" style="width: 50%;" />
</p>

## Setup

```shell
# clone it
git clone https://github.com/Taiwing/camagru
# build (this is reaaaally long, like 5 minutes)
docker compose up
```

Click [here](http://localhost:8080) to test it.

## How it works

This application is bundled with docker-compose. Each part is a service in the
compose.yaml file at the root of the repository. The pictures are stored in the
front/pictures/ directory and are accessible to the api via a shared volume.

### api

This is the backend api which handles every user or data related requests. It is
a simple HTTP server implemented in Rust, with the Rocket library, and it can be
accessed on `localhost:3000`.

### db

A Postgresql database storing user data and listing uploaded pictures.

### front

This is the user interface. A simple apache server running on `localhost:8080`
and serving html/CSS/Javascript files.

## Development

Use scripts/dev.bash to build and start this application in dev mode. The main
difference with the setup method above is that the api is run locally instead
of in a container. This is way faster to rebuild (like 10 seconds as opposed to
5 minutes for the docker way).
