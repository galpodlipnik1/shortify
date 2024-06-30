use axum::{
    extract::{Extension, Query},
    Json as AxumJson,
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
) -> Result<AxumJson<serde_json::Value>, Infallible> {
    let url = match payload.url {
        Some(url) if !url.is_empty() => url,
        _ => {
            
            return Ok(AxumJson(json!({ "error": "URL is required and cannot be empty" })));
        }
    };

    let all_keys = match db::get_all_keys(&pool).await {
        Ok(keys) => keys,
        Err(_) => {
            
            return Ok(AxumJson(json!({ "error": "Failed to get all keys" })));
        }
    };

    let mut key = nanoid!(5);
    let mut retries = 0;
    while all_keys.contains(&key) {
        key = nanoid!(5);
        retries += 1;
        if retries > 10 {
            
            return Ok(AxumJson(json!({ "error": "Failed to generate a unique key" })));
        }
    }

    let url_mapping = db::UrlMappingCreate { url, key };

    match db::create(&url_mapping, &pool).await {
        Ok(_) => {
            
            Ok(AxumJson(json!({"message": "URL created"})))
        },
        Err(_) => {
            
            Ok(AxumJson(json!({ "error": "Failed to create URL" })))
        }
    }
}