-- Set params
SET SESSION test.n_accounts = '50';
SET SESSION test.n_pictures = '500';
SET SESSION test.n_likes = '1000';
SET SESSION test.n_comments = '1000';
SET SESSION test.date_min = '2022-01-01 00:00:00';
SET SESSION test.date_max = '2022-06-11 00:00:00';
SET SESSION test.password_hash = '1e623de0d44a6e7443ab927559c02452';

-- Generate Users
INSERT INTO accounts
SELECT uuid_generate_v4(),
	CONCAT('User', id, '@lolmail.com'), 
	CONCAT('User', id),
	CURRENT_SETTING('test.password_hash'),
	true
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_accounts')::int) as id;

-- Random User function
CREATE FUNCTION random_user()
	RETURNS UUID
	LANGUAGE PLPGSQL
AS
$$
	DECLARE account UUID;
	BEGIN
		SELECT account_id INTO account FROM accounts ORDER BY RANDOM() LIMIT 1;
		RETURN account;
	END
$$;

-- Random Date function
CREATE FUNCTION random_date(start_date VARCHAR, end_date VARCHAR)
	RETURNS TIMESTAMPTZ
	LANGUAGE PLPGSQL
AS
$$
	DECLARE rand_date TIMESTAMP;
	BEGIN
		SELECT
			TO_TIMESTAMP(start_date, 'YYYY-MM-DD HH24:MI:SS') +
			random() * (TO_TIMESTAMP(end_date, 'YYYY-MM-DD HH24:MI:SS') 
			- TO_TIMESTAMP(start_date, 'YYYY-MM-DD HH24:MI:SS'))
		INTO rand_date;
		RETURN rand_date::TIMESTAMPTZ;
	END
$$;

-- Random Picture function
CREATE FUNCTION random_picture()
	RETURNS UUID
	LANGUAGE PLPGSQL
AS
$$
	DECLARE picture UUID;
	BEGIN
		SELECT picture_id INTO picture FROM pictures ORDER BY RANDOM() LIMIT 1;
		RETURN picture;
	END
$$;

-- Generate Pictures
INSERT INTO pictures
SELECT
	uuid_generate_v4(),
	random_user(),
	random_date(CURRENT_SETTING('test.date_min'), CURRENT_SETTING('test.date_max'))
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_pictures')::int);

-- Generate Likes
INSERT INTO likes
SELECT
	random_picture(),
	random_user()
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_likes')::int)
ON CONFLICT DO NOTHING;

-- Generate Comments
INSERT INTO comments
SELECT
	random_picture(),
	random_user(),
	random_date(CURRENT_SETTING('test.date_min'), CURRENT_SETTING('test.date_max')),
	md5(random()::TEXT)
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_comments')::int)
ON CONFLICT DO NOTHING;
