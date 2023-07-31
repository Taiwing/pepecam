# Pepecam

Snapchat-like web application corresponding to 42's Camagru project. Users can
login, take pictures through their webcam, upload pictures and add custom items
on them. They can comment and like each other's pictures and delete their own
posts. This is meant to be usable on desktop, phones and small resolutions.

<br />
<p align="center">
	<img src="https://github.com/Taiwing/pepecam/blob/master/front/pictures/superposables/smirk.png?raw=true" alt="Smirking Pepe" style="width: 50%;" />
</p>

You can click [here](https://pepecam.defoy.tech) to try the live application.

## Setup

```shell
# clone it
git clone https://github.com/Taiwing/pepecam

# build (the first time is reaaaally long, like 5 minutes)
./run.bash
```

Click [here](http://localhost:8080) to test it locally.

> This will work fine although the mailing functions will be disabled by default
> in the api. This is because the api needs SMTP credentials to send mails.
> Checkout the relevant [environment](#secrets) section of this document to fix
> this.

### Access this application through your local network

First you have to remove or set the 'HOST\_IP\_BIND' environment variable to an
empty string so that the running containers are accessible to the outside world.
Then, if your firewall does not block the default front port (8080), you will be
able to access it on other devices connected to your local network. However the
API requests will be blocked because of CORS rules since the only known host is
localhost. To fix this, simply change the 'GLOBAL\_HOST' env variable to your
machine's local address. Rebuild with 'run.bash' and this application will be
fully accessible on every local device.

### Populate front with pictures

When you have built the project a first time you can run the
[random\_pictures.bash](scripts/random_pictures.bash) script. It will download
pictures from an online repository and generate random pepe posts from them.
Then it will put them in the `front/pictures/pepe/` directory.

> If there are some missing pictures after the first run of the script (there
> should be 1084 pictures in the front/pictures/pepe directory), you can
> relaunch it as much as necessary to complete the list.

After you have successfully generated the appropiate number of pepes you should
stop the application and remove the database volume containing postgres' data:

```bash
# stop the application
docker compose down # or follow the script instructions if you used run.bash
# remove the pgdata volume
docker volume rm pepecam_pgdata
```

Once this is done you can relaunch the application to see the effect. Make sure
you have the POPULATE\_DB environment variable set to "true" (which should be
the case by default). When you have successfully populated the database you
should unset or remove the POPULATE\_DB variable from the [.env](.env) file.
Otherwise the populate scripts are going to run each time you relaunch the
project and fill the database with more random data every time.

> If you do not download random pictures but have the database populated anyway
> you will still get random pictures in the front but without pepes. This is the
> default behavior as POPULATE\_DB is set to true by default.

## How it works

This application is bundled with docker-compose. Each part is a service in the
compose.yaml file at the root of the repository. The pictures are stored in the
front/pictures/ directory and are accessible to the api via a shared volume.

### api

This is the backend api which handles every user or data related requests. It is
a simple HTTP server implemented in Rust, with the Rocket library, and it can be
accessed on `localhost:3000` by default.

### db

A Postgresql database storing user data and listing uploaded pictures.

### front

This is the user interface. A simple apache server running on `localhost:8080`
by default and serving html/CSS/Javascript files.

## Environment

Every environment variable defined in the [.env](.env) file can be changed.
However, the variables containing '$' substitutions should not be modified
directly. Instead it is the variables they refer to that should be changed.

### Secrets

As mentioned in the [Setup](#setup) section, the SMTP credentials are not
provided in the default environment file. The application will run with this
environment but it will not be able to send emails because of the missing env
values (the 'Secrets' section of the [.env](.env) file). This means that you
wont be able to register new users or use the password reset functions.

To fix this you will have to give a value to these missing variables:
- SMTP\_SERVER
- SMTP\_PORT
- SMTP\_USERNAME
- SMTP\_PASSWORD

Of course this means that you will have to setup your own SMTP server or
register to a third party service. You can use [Brevo's](https://www.brevo.com/)
free plan which is more than enough for testing purposes.

### Global

The Global variables can all be changed by the user. They will apply to the
entire application. For example, if you change FRONT\_PUBLIC\_PORT the front
will be served on a new port and the api will send its emails using the new
FRONT\_LINK value instead of the default.

> Be careful if changing the '\_DIR'-suffixed variables. They refer to actual
> files in the front/ directory and are used for a shared volume in the compose
> configuration. They should match the front/pictures/ layout.

### API

Only applies to the api. 'RUST\_' and 'ROCKET\_'-prefixed variables are compile
time config to build the api binary. The other variables refer to api constants
and can be changed at will.

### DB

Only applies to the db. The most important variable is 'POPULATE\_DB'. If it is
set to a non-empty string the [populate.sql](db/populate.sql) script will be
executed. It will fill the database with users and random data making it easier
to test the application.
