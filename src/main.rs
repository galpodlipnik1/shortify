use axum::{routing::{get, post}, Extension, Router};
use handlers::post::create_url;
use handlers::get::{get_url, get_id, get_all, get_all_key_url,  reroute};

mod handlers;
mod db;


#[tokio::main]
async fn main() {
    let pool = db::setup_database().await.expect("Failed to setup database");

    let routes_all = Router::new()
        .merge(post_routes())
        .merge(get_routes())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn post_routes() -> Router {
    Router::new()
        .route("/create", post(create_url))
}

fn get_routes() -> Router {
    Router::new()
        .route("/getUrl/:id", get(get_url))
        .route("/getId", get(get_id))
        .route("/getAllKeys", get(get_all))
        .route("/getKeyUrl", get(get_all_key_url))
        .route("/:id", get(reroute))
}