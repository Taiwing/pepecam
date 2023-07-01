CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS accounts (
	account_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	email VARCHAR(256) NOT NULL UNIQUE,
	username VARCHAR(64) NOT NULL UNIQUE,
	password_hash VARCHAR NOT NULL,
	email_notifications BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TYPE superposable AS ENUM (
	'chic',
	'cry',
	'honk',
	'rage',
	'sad',
	'smirk',
	'stoned',
	'sweat'
);

CREATE TABLE IF NOT EXISTS pictures (
	picture_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	account_id UUID NOT NULL,
	superposable superposable NOT NULL,
	creation_ts TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pictures
	ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);

CREATE TABLE IF NOT EXISTS likes (
	picture_id UUID NOT NULL,
	account_id UUID NOT NULL,
	value BOOLEAN NOT NULL DEFAULT TRUE,
	CONSTRAINT NO_DUPLICATE_LIKE UNIQUE (picture_id, account_id)
);

ALTER TABLE likes
	ADD FOREIGN KEY (picture_id) REFERENCES pictures (picture_id)
	ON DELETE CASCADE;
ALTER TABLE likes
	ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);

CREATE TABLE IF NOT EXISTS comments (
	picture_id UUID NOT NULL,
	account_id UUID NOT NULL,
	creation_ts TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	content VARCHAR(512) NOT NULL
);

ALTER TABLE comments
	ADD FOREIGN KEY (picture_id) REFERENCES pictures (picture_id)
	ON DELETE CASCADE;
ALTER TABLE comments
	ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);
