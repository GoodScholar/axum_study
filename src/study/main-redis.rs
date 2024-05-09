#![allow(unused)]
use axum::{
    routing::{get, post},
    Form, Json, Router,
};
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tower_http::trace::TraceLayer;

const REDIS_DSN: &str = "redis://127.0.0.1:6379/";

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn set_item() -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| err.to_string())?;
    conn.set("author", "axum.rs")
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully set")
}

async fn get_item() -> Result<String, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| err.to_string())?;
    let value = conn.get("author").await.map_err(|err| err.to_string())?;
    Ok(value)
}

async fn set_user() -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| err.to_string())?;
    let user = UserInfo {
        id: 1,
        username: "axum.rs".to_string(),
        email: "team@axum.rs".to_string(),
    };
    let user = json!(user);
    conn.set("user", user.to_string())
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully set user.")
}
async fn get_user() -> Result<Json<UserInfo>, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| err.to_string())?;
    let value: String = conn.get("user").await.map_err(|err| err.to_string())?;
    let user: UserInfo = from_str(&value).map_err(|err| err.to_string())?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug,middleware=debug");
    }
    tracing_subscriber::fmt::init();

    let routes = Router::new()
        .route("/set", get(set_item))
        .route("/get", get(get_item))
        .route("/set_user", get(set_user))
        .route("/get_user", get(get_user))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
