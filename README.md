# Pepecam

Snapchat-like web application corresponding to 42's Camagru project. Users can
login, take pictures through their webcam, upload pictures and add custom items
on them. They can comment and like each other's pictures and delete their own
posts. This is meant to be usable on desktop, phones and small resolutions.

<br />
<p align="center">
	<img src="https://github.com/Taiwing/camagru/blob/master/front/pictures/superposables/smirk.png?raw=true" alt="Smirking Pepe" style="width: 50%;" />
</p>

## Setup

```shell
# clone it
git clone https://github.com/Taiwing/camagru

# decrypt .env file if you have the password (see below for more info)
transcrypt -c aes-256-cbc -p TRANSCRYPT_PASSWORD
# OR replace encrypted .env file with default
mv .env.template .env

# build (the first time is reaaaally long, like 5 minutes)
./run.bash
```

> The complete environment file (.env) is available in this repository. It
> contains API credentials (only SMTP for now) and is encrypted with
> [transcrypt](https://github.com/elasticdog/transcrypt) for obvious security
> reasons. If you do not have the TRANSCRYPT\_PASSWORD you can simply remove
> the encrypted .env file and replace it with the .env.template.

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

## Environment

Every environment variable defined in the .env file can be changed. However, the
variables containing '$' substitutions should not be changed directly. Instead
it is the variables they refer to that should be changed.

### Secrets

As mentioned in the Setup section, you must have transcrypt installed and
configured to use the default .env file. If you do not have it you will have
to use the [.env.template](.env.template) instead. The application will run with
this environment but it will not be able to send emails because of the missing
env values (the 'Secrets' section of the .env.template file). This means that
you wont be able to register new users or to use the password reset functions.

To fix this you will have to give a value to these missing variables:
- SMTP\_SERVER
- SMTP\_PORT
- SMTP\_USERNAME
- SMTP\_PASSWORD

Of course this means that you will have to setup your own SMTP server or
register to a third party service. You can use [Brevo's](https://www.brevo.com/)
free plan which is more than enough for testing purposes.

### Constants

These values can be changed but they will require manual modifications in some
places. This is mainly in the front because we cannot directly pass environment
variables to the client context. There are some ways to deal with this like by
using server-side rendering or setting up an endpoint dedicated to this.
However that looks like too much work for four measly variables.

> Be careful if changing the '\_DIR'-suffixed variables. They refer to actual
> files in the front/ directory and are used for a shared volume in the compose
> configuration. They should match the front/pictures/ layout.

### Global

The Global variables can all be changed by the user. They will apply to the
entire application. For example, if you change FRONT\_PORT the front will be
served on a new port and the api will send its emails using the new FRONT\_LINK
value instead of the default.

### API

Only applies to the api. 'RUST\_' and 'ROCKET\_'-prefixed variables are compile
time config to build the api binary. The other variables refer to api constants
and can be changed at will.

### DB

Only applies to the db. The most important variable is 'POPULATE\_DB'. If it is
set to a non-empty string the [populate.sql](db/populate.sql) script will be
executed. It will fill the database with users and random data making it easier
to test the application.
