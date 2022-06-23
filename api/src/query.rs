//! Handle every sql query for the api.

use crate::rocket::futures::TryStreamExt;
use crate::{password, payload::NewUser};
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{types::Uuid, PgPool, Row};

//TODO: find a way to remove the "postgres" string or to use the environment
//instead (something like 'std::env!("DATABASE_NAME")' if possible).
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
    match row {
        None => false,
        Some(_) => true,
    }
}

/// Get a list of ids for every picture in the database.
pub async fn pictures(mut db: Connection<PostgresDb>) -> Option<Vec<String>> {
    let mut rows =
        sqlx::query("SELECT picture_id FROM pictures;").fetch(&mut *db);
    let mut pictures: Vec<String> = vec![];
    while let Some(row) = rows.try_next().await.ok()? {
        let picture_id: Uuid = row.try_get(0).ok()?;
        pictures.push(picture_id.to_hyphenated().to_string());
    }
    match pictures.len() {
        0 => None,
        _ => Some(pictures),
    }
}

/// Get a list of pictures uploaded by a given user.
pub async fn user_pictures(
    mut db: Connection<PostgresDb>,
    username: &str,
) -> Option<Vec<String>> {
    let mut rows = sqlx::query(
        "
		SELECT picture_id
		FROM pictures JOIN accounts ON accounts.username = $1
		WHERE accounts.account_id = pictures.account_id;
	",
    )
    .bind(username)
    .fetch(&mut *db);
    let mut pictures: Vec<String> = vec![];
    while let Some(row) = rows.try_next().await.ok()? {
        let picture_id: Uuid = row.try_get(0).ok()?;
        pictures.push(picture_id.to_hyphenated().to_string());
    }
    match pictures.len() {
        0 => None,
        _ => Some(pictures),
    }
}

/// Create an account for a new user.
pub async fn create_account(
    mut db: Connection<PostgresDb>,
    new_user: NewUser,
) -> Result<String, sqlx::Error> {
    let password_hash = password::hash(&new_user.password);
    println!("password_hash: '{}'", &password_hash);
    println!(
        "password::verify output: '{}'",
        password::verify(&new_user.password, &password_hash)
    );
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
    .fetch_one(&mut *db)
    .await?;
    Ok(format!(
        "Great success! New user account '{}' has been created!",
        new_user.username
    ))
}
