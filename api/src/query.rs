//! Handle every sql query for the api.

use crate::uuid::{from_sqlx_to_serde, SqlxUuid};
use crate::{
    auth::password,
    payload::{NewUser, Picture},
};
use rocket_db_pools::sqlx::{self, PgPool};
use rocket_db_pools::{Connection, Database};

pub mod types {
    use super::sqlx::{self, types::time::OffsetDateTime};
    use super::SqlxUuid;

    /// An account instance from the 'accounts' table.
    #[derive(sqlx::FromRow)]
    pub struct Account {
        pub account_id: SqlxUuid,
        pub email: String,
        pub username: String,
        pub password_hash: String,
        pub email_notifications: bool,
    }

    /// A picture from the GET pictures request
    #[derive(sqlx::FromRow)]
    pub struct DbPicture {
        pub picture_id: SqlxUuid,
        pub account_id: SqlxUuid,
        pub creation_ts: OffsetDateTime,
        pub author: String,
        pub like_count: i64,
        pub dislike_count: i64,
        pub comment_count: i64,
        pub liked: Option<bool>,
        pub disliked: Option<bool>,
    }

    /// A picture id from the POST picture request
    #[derive(sqlx::FromRow)]
    pub struct PictureId {
        pub picture_id: SqlxUuid,
    }
}

//TODO: find a way to remove the "postgres" string or to use the environment
//instead (something like 'std::env!("DATABASE_NAME")' if possible).
//TODO: Maybe actually implement this structure. This would mean setting every
//query function as a mehod of a PostgresDb implementation (which would all work
//on an '&mut PostgresDb' instance or something). This would be easier to use.
#[derive(Database)]
#[database("postgres")]
pub struct PostgresDb(PgPool);

/// Check if the given field value is already present in the accounts table.
pub async fn is_taken(
    field: &str,
    value: &str,
    db: &mut Connection<PostgresDb>,
) -> bool {
    let query = format!("SELECT {} FROM accounts WHERE {} = $1;", field, field);
    let row = sqlx::query(&query)
        .bind(value)
        .fetch_optional(&mut **db)
        .await
        .unwrap();
    row.is_some()
}

/// Check if the given user account exists
pub async fn account_exists(
    account_id: &SqlxUuid,
    db: &mut Connection<PostgresDb>,
) -> bool {
    let query = "SELECT account_id FROM accounts WHERE account_id = $1;";
    let row = sqlx::query(query)
        .bind(account_id)
        .fetch_optional(&mut **db)
        .await
        .unwrap();
    row.is_some()
}

/// Get a list of ids for every picture in the database.
pub async fn pictures(
    db: &mut Connection<PostgresDb>,
    index: u32,
    count: u32,
    connected_user: Option<SqlxUuid>,
) -> Option<Vec<Picture>> {
    let query = "
		SELECT
			pictures.picture_id, pictures.account_id, pictures.creation_ts,
			accounts.username as author,
			COUNT(CASE WHEN likes.value = TRUE THEN 1 END) AS like_count,
			COUNT(CASE WHEN likes.value = FALSE THEN 1 END) AS dislike_count,
			COALESCE(comment_counts.comment_count, 0) AS comment_count,
			BOOL_OR(likes.value = TRUE AND likes.account_id = $3) AS liked,
			BOOL_OR(likes.value = FALSE AND likes.account_id = $3) AS disliked
		FROM pictures
		JOIN accounts ON pictures.account_id = accounts.account_id
		LEFT JOIN likes ON pictures.picture_id = likes.picture_id
		LEFT JOIN (
			SELECT picture_id, COUNT(*) AS comment_count
			FROM comments GROUP BY picture_id
		) AS comment_counts ON pictures.picture_id = comment_counts.picture_id
		GROUP BY
			pictures.picture_id, accounts.username, comment_counts.comment_count
		ORDER BY creation_ts DESC LIMIT $1 OFFSET $2;
	";

    let raw_pictures = sqlx::query_as::<_, types::DbPicture>(query)
        .bind(count)
        .bind(index * count)
        .bind(connected_user)
        .fetch_all(&mut **db)
        .await
        .unwrap();
    if raw_pictures.is_empty() {
        return None;
    }

    let pictures = raw_pictures
        .iter()
        .map(|raw_picture| Picture {
            picture_id: from_sqlx_to_serde(&raw_picture.picture_id),
            account_id: from_sqlx_to_serde(&raw_picture.account_id),
            creation_ts: raw_picture.creation_ts.unix_timestamp(),
            author: raw_picture.author.clone(),
            like_count: raw_picture.like_count,
            dislike_count: raw_picture.dislike_count,
            comment_count: raw_picture.comment_count,
            liked: raw_picture.liked,
            disliked: raw_picture.disliked,
        })
        .collect();
    Some(pictures)
}

/// Get a list of pictures uploaded by a given user.
pub async fn user_pictures(
    db: &mut Connection<PostgresDb>,
    username: &str,
    index: u32,
    count: u32,
) -> Option<Vec<Picture>> {
    let query = "
		SELECT
			pictures.picture_id, pictures.account_id, pictures.creation_ts,
			accounts.username as author,
			COUNT(CASE WHEN likes.value = TRUE THEN 1 END) AS like_count,
			COUNT(CASE WHEN likes.value = FALSE THEN 1 END) AS dislike_count,
			COALESCE(comment_counts.comment_count, 0) AS comment_count
		FROM pictures
		JOIN accounts
		ON pictures.account_id = accounts.account_id AND accounts.username = $1
		LEFT JOIN likes ON pictures.picture_id = likes.picture_id
		LEFT JOIN (
			SELECT picture_id, COUNT(*) AS comment_count
			FROM comments GROUP BY picture_id
		) AS comment_counts ON pictures.picture_id = comment_counts.picture_id
		GROUP BY
			pictures.picture_id, accounts.username, comment_counts.comment_count
		ORDER BY creation_ts DESC LIMIT $2 OFFSET $3;
	";

    let raw_pictures = sqlx::query_as::<_, types::DbPicture>(query)
        .bind(username)
        .bind(count)
        .bind(index * count)
        .fetch_all(&mut **db)
        .await
        .unwrap();
    if raw_pictures.is_empty() {
        return None;
    }

    let pictures = raw_pictures
        .iter()
        .map(|raw_picture| Picture {
            picture_id: from_sqlx_to_serde(&raw_picture.picture_id),
            account_id: from_sqlx_to_serde(&raw_picture.account_id),
            creation_ts: raw_picture.creation_ts.unix_timestamp(),
            author: raw_picture.author.clone(),
            like_count: raw_picture.like_count,
            dislike_count: raw_picture.dislike_count,
            comment_count: raw_picture.comment_count,
            liked: None,
            disliked: None,
        })
        .collect();
    Some(pictures)
}

/// Create an account for a new user.
pub async fn create_account(
    db: &mut Connection<PostgresDb>,
    new_user: &NewUser,
) -> Result<String, sqlx::Error> {
    let password_hash = password::hash(&new_user.password);
    sqlx::query(
        "
		INSERT INTO accounts (email, username, password_hash)
		VALUES ($1, $2, $3)
		RETURNING account_id;
	",
    )
    .bind(&new_user.email)
    .bind(&new_user.username)
    .bind(&password_hash)
    .fetch_one(&mut **db)
    .await?;
    Ok(format!(
        "Great success! New user account '{}' has been created!",
        new_user.username
    ))
}

/// Get user by username
pub async fn get_user_by_username(
    username: &str,
    db: &mut Connection<PostgresDb>,
) -> Option<types::Account> {
    let query = "SELECT * FROM accounts WHERE username = $1;";
    sqlx::query_as(query)
        .bind(username)
        .fetch_optional(&mut **db)
        .await
        .unwrap()
}

/// Modify user account
///
/// Note: This implementation is really dumb. It executes one query per
/// optional parameter. There should be an elegant way of bulding a multi
/// type dynamic query but I did not find it. This is the simplest way.
pub async fn put_user(
    db: &mut Connection<PostgresDb>,
    account_id: &SqlxUuid,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    email_notifications: Option<bool>,
) -> Result<(), sqlx::Error> {
    let password_hash = password.map(|password| password::hash(&password));

    if let Some(username) = username {
        let query = "UPDATE accounts SET username = $1 WHERE account_id = $2";
        sqlx::query(query)
            .bind(&username)
            .bind(account_id)
            .execute(&mut **db)
            .await?;
    }

    if let Some(password_hash) = password_hash {
        let query =
            "UPDATE accounts SET password_hash = $1 WHERE account_id = $2";
        sqlx::query(query)
            .bind(&password_hash)
            .bind(account_id)
            .execute(&mut **db)
            .await?;
    }

    if let Some(email) = email {
        let query = "UPDATE accounts SET email = $1 WHERE account_id = $2";
        sqlx::query(query)
            .bind(&email)
            .bind(account_id)
            .execute(&mut **db)
            .await?;
    }

    if let Some(email_notifications) = email_notifications {
        let query = "UPDATE accounts SET email_notifications = $1 WHERE account_id = $2";
        sqlx::query(query)
            .bind(&email_notifications)
            .bind(account_id)
            .execute(&mut **db)
            .await?;
    }

    Ok(())
}

/// Add a like or a dislike on a given picture
pub async fn put_like(
    db: &mut Connection<PostgresDb>,
    like: bool,
    picture_id: &SqlxUuid,
    account_id: &SqlxUuid,
) -> Result<(), sqlx::Error> {
    let query = "
		INSERT INTO likes (picture_id, account_id, value)
		VALUES ($1, $2, $3)
		ON CONFLICT ON CONSTRAINT no_duplicate_like
		DO UPDATE SET value = $3;
	";

    sqlx::query(query)
        .bind(picture_id)
        .bind(account_id)
        .bind(like)
        .execute(&mut **db)
        .await?;
    Ok(())
}

/// Remove like or dislike on a given picture
pub async fn delete_like(
    db: &mut Connection<PostgresDb>,
    picture_id: &SqlxUuid,
    account_id: &SqlxUuid,
) -> Result<(), sqlx::Error> {
    let query = "DELETE FROM likes WHERE picture_id = $1 AND account_id = $2";

    sqlx::query(query)
        .bind(picture_id)
        .bind(account_id)
        .execute(&mut **db)
        .await?;
    Ok(())
}

/// Add given comment to a picture
pub async fn comment(
    db: &mut Connection<PostgresDb>,
    comment: &str,
    picture_id: &SqlxUuid,
    account_id: &SqlxUuid,
) -> Result<(), sqlx::Error> {
    let query = "
		INSERT INTO comments (picture_id, account_id, content)
		VALUES ($1, $2, $3);
	";

    sqlx::query(query)
        .bind(picture_id)
        .bind(account_id)
        .bind(comment)
        .execute(&mut **db)
        .await?;
    Ok(())
}

/// Create a new picture
pub async fn post_picture(
    db: &mut Connection<PostgresDb>,
    account_id: &SqlxUuid,
) -> Result<SqlxUuid, sqlx::Error> {
    let query = "
		INSERT INTO pictures (account_id) VALUES ($1)
		RETURNING picture_id;
	";

    let new_picture: types::PictureId = sqlx::query_as(query)
        .bind(account_id)
        .fetch_one(&mut **db)
        .await?;
    Ok(new_picture.picture_id)
}

/// Delete a picture
pub async fn delete_picture(
    db: &mut Connection<PostgresDb>,
    picture_id: &SqlxUuid,
    account_id: &SqlxUuid,
) -> Result<u64, sqlx::Error> {
    let query =
        "DELETE FROM pictures WHERE picture_id = $1 AND account_id = $2";

    sqlx::query(query)
        .bind(picture_id)
        .bind(account_id)
        .execute(&mut **db)
        .await
        .map(|result| result.rows_affected())
}
