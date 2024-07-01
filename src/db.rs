use std::{error::Error, vec};
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{query, Row};
use futures::stream::TryStreamExt;

#[derive(Debug, Deserialize)]
pub struct UrlMappingCreate {
    pub url: String,
    pub key: String,
}

pub async fn setup_database() -> Result<sqlx::PgPool, Box<dyn Error>> {
    dotenv().ok();

    let url = env::var("DB_CONNECTION").expect("DB_CONNECTION must be set");

    let pool = sqlx::PgPool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn create(url_mapping: &UrlMappingCreate, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO UrlMapping (url, key) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(&url_mapping.url)
        .bind(&url_mapping.key)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_id_for_url(url: &str, conn: sqlx::PgPool) -> Result<Option<String>, Box<dyn Error>> {
    let q = "SELECT key FROM UrlMapping WHERE url = $1";
    let query = query(q).bind(url);

    let row = query.fetch_optional(&conn).await?;

    match row {
        Some(row) => Ok(row.get(0)),
        None => Ok(None),
    }
}

pub async fn get_url_for_id(key: &str, conn: sqlx::PgPool) -> Result<Option<String>, Box<dyn Error>> {
    let q = "SELECT url FROM UrlMapping WHERE key = $1";
    let query = query(q).bind(key);

    let row = query.fetch_optional(&conn).await?;

    match row {
        Some(row) => Ok(row.get(0)),
        None => Ok(None),
    }
}

pub async fn get_all_keys(conn: &sqlx::PgPool) -> Result<Vec<String>, Box<dyn Error>> {
    let q = "SELECT key FROM UrlMapping";
    let query = query(q);

    let mut rows = query.fetch(conn);

    let mut keys = vec![];

    while let Some(row) = rows.try_next().await? {
        keys.push(row.get(0));
    }
    Ok(keys)
}

pub async fn get_all_key_url(conn: &sqlx::PgPool) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let q = "SELECT key, url FROM UrlMapping";
    let query = query(q);

    let mut rows = query.fetch(conn);

    let mut key_urls = vec![];

    while let Some(row) = rows.try_next().await? {
        key_urls.push((row.get(0), row.get(1)));
    }
    Ok(key_urls)
}