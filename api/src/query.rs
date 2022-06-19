use crate::rocket::futures::TryStreamExt;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{types::Uuid, PgPool, Row};

//TODO: find a way to remove the "postgres" string or to use the environment
//instead (something like 'std::env!("DATABASE_NAME")' if possible).
#[derive(Database)]
#[database("postgres")]
pub struct PostgresDb(PgPool);

pub async fn is_taken(
    field: &str,
    value: &str,
    db: &mut Connection<PostgresDb>,
) -> bool {
    let query = format!("SELECT {} FROM accounts WHERE {} = $1", field, field);
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
