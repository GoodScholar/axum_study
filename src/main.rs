#![allow(unused)]

use std::env;

use axum::{
    extract::Multipart,
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

use dotenv::dotenv;
use serde::Deserialize;
use tower_http::trace::TraceLayer;

mod logger;

// 上传文件的页面
async fn index() {
    println!("123123123")
}

/// Web配置
#[derive(Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub addr: String,
}

/// Redis 配置
#[derive(Deserialize)]
pub struct RedisConfig {
    /// 连接字符串
    pub dsn: String,
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    logger::init_logger();

    dotenv().ok();
    let web_addr = env::var("WEB_ADDR").expect("WEB_ADDR is not set.");

    let routes = Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(web_addr).await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
