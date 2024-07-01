use axum::{
    extract::{Extension, Query}, http::HeaderMap, Json as AxumJson
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use nanoid::nanoid;
use std::convert::Infallible;

use crate::db;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUrl {
    url: Option<String>,
}

pub async fn create_url(
    Query(payload): Query<CreateUrl>,
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
) -> Result<AxumJson<serde_json::Value>, Infallible> {
    let url = match payload.url {
        Some(url) if !url.is_empty() => url,
        _ => return Ok(AxumJson(json!({ "error": "URL is required and cannot be empty" }))),
    };

    let all_keys_url = match db::get_all_key_url(&pool).await {
        Ok(keys) => keys,
        Err(_) => return Ok(AxumJson(json!({ "error": "Failed to get all keys" }))),
    };

    if let Some((existing_key, _)) = all_keys_url.iter().find(|(_, existing_url)| existing_url == &url) {
        let host = headers.get("host").unwrap().to_str().unwrap();
        return Ok(AxumJson(json!({
            "message": "URL already exists",
            "key": existing_key,
            "shortUrl": format!("http://{}/{}", host, existing_key)
        })));
    }

    let mut key = nanoid!(5);
    let mut retries = 0;
    while all_keys_url.iter().any(|(existing_key, _)| existing_key == &key) {
        key = nanoid!(5);
        retries += 1;
        if retries > 10 {
            return Ok(AxumJson(json!({ "error": "Failed to generate a unique key" })));
        }
    }

    let key_clone = key.clone();
    let host = headers.get("host").unwrap().to_str().unwrap();
    let url_mapping = db::UrlMappingCreate { url, key };

    match db::create(&url_mapping, &pool).await {
        Ok(_) => Ok(AxumJson(json!({
            "message": "URL created",
            "key": key_clone,
            "shortUrl": format!("http://{}/{}", host, key_clone)
        }))),
        Err(_) => Ok(AxumJson(json!({ "error": "Failed to create URL" }))),
    }
}