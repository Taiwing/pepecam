CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS accounts (
	account_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	email VARCHAR(256) NOT NULL,
	username VARCHAR(64) NOT NULL,
	password_hash VARCHAR NOT NULL,
	email_notifications BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS pictures (
	picture_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	account_id UUID NOT NULL,
	creation_ts DATE NOT NULL DEFAULT CURRENT_DATE
);

ALTER TABLE pictures ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);

CREATE TABLE IF NOT EXISTS likes (
	picture_id UUID NOT NULL,
	account_id UUID NOT NULL,
	CONSTRAINT NO_DUPLICATE_LIKE UNIQUE (picture_id, account_id)
);

ALTER TABLE likes ADD FOREIGN KEY (picture_id) REFERENCES pictures (picture_id);
ALTER TABLE likes ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);

CREATE TABLE IF NOT EXISTS comments (
	picture_id UUID NOT NULL,
	account_id UUID NOT NULL,
	creation_ts DATE NOT NULL DEFAULT CURRENT_DATE,
	content VARCHAR(512) NOT NULL
);

ALTER TABLE comments ADD FOREIGN KEY (picture_id) REFERENCES pictures (picture_id);
ALTER TABLE comments ADD FOREIGN KEY (account_id) REFERENCES accounts (account_id);
