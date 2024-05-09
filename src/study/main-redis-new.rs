#![allow(unused)]
use std::{fmt::Display, str::FromStr};

use axum::{
    routing::{get, post},
    Form, Json, Router,
};
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tower_http::trace::TraceLayer;

mod logger;
mod redis_client;

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn set_item() -> Result<&'static str, String> {
    redis_client::write_to_redis("author", "axum.rs").await?;
    redis_client::write_ex_to_redis("my_key", "13131231231232", 3).await?;

    Ok("Successfully set")
}

async fn get_item() -> Result<String, String> {
    let value: String = redis_client::read_from_redis("author").await?;
    Ok(value)
}
async fn get_key() -> Result<String, String> {
    let value: String = redis_client::read_from_redis("my_key").await?;
    Ok(value)
}

async fn set_user() -> Result<&'static str, String> {
    let user = UserInfo {
        id: 1,
        username: "axum.rs".to_string(),
        email: "team@axum.rs".to_string(),
    };

    redis_client::write_to_redis("user", json!(user).to_string()).await?;
    Ok("Successfully set user.")
}
async fn get_user() -> Result<Json<UserInfo>, String> {
    let value: String = redis_client::read_from_redis("user").await?;
    let user: UserInfo = from_str(&value).map_err(|err| err.to_string())?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    logger::init_logger();

    let routes = Router::new()
        .route("/set", get(set_item))
        .route("/get", get(get_item))
        .route("/get_key", get(get_key))
        .route("/set_user", get(set_user))
        .route("/get_user", get(get_user))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
