#![allow(unused)]

use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query},
    routing::{get, post},
    Extension, Router,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::add_extension::AddExtensionLayer;

#[derive(Clone)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Clone)]
pub struct DatabaseClient {
    pub dsn: String,
}
pub struct RedisClient {
    pub host: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub rdb: Arc<RedisClient>,
}

pub struct ArcState {
    pub db: DatabaseClient,
    pub rdb: RedisClient,
}

#[tokio::main]
async fn main() {
    let db_client = DatabaseClient {
        dsn: "host=pg.axum.rs port=5432 user=axum_rs password=axum.rs sslmode=disable".to_string(),
    };

    let redis_client = Arc::new(RedisClient {
        host: "redis.axum.rs".to_string(),
    });

    let redis_client_arc = RedisClient {
        host: "redis.axum.arc.rs".to_string(),
    };

    let routes = Router::new()
        .route("/user", get(show_user_info))
        .route("/user_arc", get(show_user_info_arc))
        .route("/status", get(status))
        .route("/status_arc", get(status_arc))
        // .layer(AddExtensionLayer::new(UserInfo {
        //     username: "axum.rs".to_string(),
        // }));
        // .layer(AddExtensionLayer::new(Arc::new(UserInfo {
        //     username: "axum.rs".to_string(),
        // })));
        // .layer(AddExtensionLayer::new(AppState {
        //     db: db_client,
        //     rdb: redis_client,
        // }));
        .layer(AddExtensionLayer::new(Arc::new(ArcState {
            db: db_client,
            rdb: redis_client_arc,
        })));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

async fn show_user_info(Extension(info): Extension<UserInfo>) -> String {
    format!("Sigined User: {}", info.username)
}

async fn show_user_info_arc(Extension(info): Extension<Arc<UserInfo>>) -> String {
    format!("Sigined Arc User: {}", info.username)
}

async fn status(Extension(state): Extension<AppState>) -> String {
    format!(
        "database dsn: {}, redis host: {}",
        state.db.dsn, state.rdb.host
    )
}

async fn status_arc(Extension(state): Extension<Arc<ArcState>>) -> String {
    format!(
        "database dsn: {}, redis host: {}",
        state.db.dsn, state.rdb.host
    )
}
