use axum::{extract::{Path, Query}, http::{self, StatusCode}, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use db::{get_all_keys, get_url_for_id, get_id_for_url};

use crate::db;

#[derive(Deserialize, Debug)]
pub struct GetUrl {
    id: Option<String>, 
}

#[derive(Deserialize, Debug)]
pub struct GetId {
    url: Option<String>, 
}

pub async fn get_url(
    Path(GetUrl { id }): Path<GetUrl>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    match id {
        Some(id) => {
            match get_url_for_id(&id, pool).await { 
                Ok(url) => Json(json!({"success": true, "url": url})), 
                Err(_) => Json(json!({"success": false, "error": "URL not found"})), 
            }
        },
        None => Json(json!({"success": false, "error": "No ID provided"})), 
    }
}

pub async fn get_id(
    Query(GetId { url }): Query<GetId>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    match url {
        Some(url) => {
            match get_id_for_url(&url, pool).await { 
                Ok(id) => Json(json!({"success": true, "id": id})), 
                Err(_) => Json(json!({"success": false, "error": "ID not found for URL"})), 
            }
        },
        None => Json(json!({"success": false, "error": "No URL provided"})), 
    }
}

pub async fn get_all(Extension(pool): Extension<PgPool>,) -> impl IntoResponse {
    match get_all_keys(&pool).await {
        Ok(keys) => Json(json!({"success": true, "keys": keys})), 
        Err(_) => Json(json!({"success": false, "error": "Failed to retrieve keys"})), 
    }
}

pub async fn get_all_key_url(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match db::get_all_key_url(&pool).await {
        Ok(key_urls) => Json(json!({"success": true, "key_urls": key_urls})),
        Err(_) => Json(json!({"success": false, "error": "Failed to retrieve key URLs"})),
    }
}

pub async fn reroute(
    Path(GetUrl { id }): Path<GetUrl>,
    Extension(pool): Extension<PgPool>
) -> impl IntoResponse {
    if id.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "ID is required"
        ).into_response();
    }

    let id_str = id.as_ref().unwrap();

    match get_url_for_id(id_str, pool).await {
        Ok(url) => match url {
            Some(url) => {
                println!("Redirecting to: {}", url);
                return http::Response::builder()
                    .status(StatusCode::FOUND)
                    .header("Location", url)
                    .body("Redirecting...".into())
                    .unwrap();
            },
            None => (
                StatusCode::NOT_FOUND,
                "URL not found"
            ).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve URL"
        ).into_response(),
    }
}