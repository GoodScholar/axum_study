#![allow(unused)]

use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use bytes::Bytes;
use serde::Serialize;
use serde_json::{json, Value};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/str", get(str_response))
        .route("/string", get(string_response))
        .route("/404", get(not_found))
        .route("/headers", get(with_headers))
        .route("/status", get(with_headers_and_status))
        .route("/html", get(html))
        .route("/json", get(json))
        .route("/result", get(result))
        .route("/struct", get(info_struct))
        .route("/err", get(app_error))
        .route("/cn", get(cn));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

async fn str_response() -> &'static str {
    "Hello, axum.rs"
}
async fn string_response() -> String {
    "Hello, axum.rs".to_string()
}
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum.rs"),
    );
    (headers, "axum.rs")
}

async fn with_headers_and_status() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum.rs"),
    );
    (StatusCode::OK, headers, "axum.rs")
}

async fn html() -> Html<&'static str> {
    Html("Hello, <em>axum.rs</em>")
}

async fn json() -> Json<Value> {
    Json(json!({"hello":"axum.rs"}))
}

async fn result() -> Result<&'static str, StatusCode> {
    let flag = false;
    if flag {
        Ok("Hello, axum.rs")
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Serialize)]
struct Info {
    web_site: String,
    email: String,
    level: i32,
}

async fn info_struct() -> Json<Info> {
    let info = Info {
        web_site: "https://axum.rs".to_string(),
        email: "team@axum.rs".to_string(),
        level: 123,
    };
    Json(info)
}

enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::SomethingElseWentWrong => "something else went wrong",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

async fn app_error() -> Result<&'static str, MyError> {
    let flag = true;
    if flag {
        Ok("Hello, axum.rs")
    } else {
        Err(MyError::SomethingWentWrong)
    }
}

// 设置中文返回，防止乱码
async fn cn() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain;charset=utf-8"),
    );
    (headers, "你好，axum.rs")
}
