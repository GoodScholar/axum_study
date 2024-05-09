#![allow(unused)]

use std::{
    fs::{self, File},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::Multipart,
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
mod logger;

/// 允许上传的大小
const MAX_REQUEST_SIZE: usize = 20 * 1024 * 1024; // 设置请求体大小限制为 20MB
const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB
const SAVE_FILE_BASE_PATH: &str = "uploads";

// 上传文件的页面
async fn upload_page() -> impl IntoResponse {
    Html(
        r#"
        <body>
            <form action="do_upload" method="post" enctype="multipart/form-data">
                <input type="file" name="uploadFile">
                <input type="submit" value="Upload">
            </form>
        <body>
        "#,
    )
}

// 将上传文件写到本地
async fn do_upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut file) = multipart.next_field().await.expect("next file failed") {
        //文件类型
        let content_type = file.content_type().unwrap().to_string();
        println!("content_type-->{}", content_type);

        // 文件名字
        let filename = file.file_name().unwrap().to_string();
        // 获取当前时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // 文件上传路径
        let filepath = format!("{}/{}_{}", SAVE_FILE_BASE_PATH, timestamp, filename);
        // 创建
        let mut upload_file = match File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return Html("Failed to create file"),
        };

        let mut total_size: u64 = 0;

        while let Some(chunk) = file.chunk().await.unwrap() {
            total_size += chunk.len() as u64;
            if total_size > 10 * 1024 * 1024 {
                // 如果文件大小超过 10MB，返回请求实体过大的错误响应
                return Html("File size exceeds the limit (10MB)!");
            }
            if let Err(_) = upload_file.write_all(&chunk) {
                return Html("Failed to write file");
            }
        }
    }

    Html("upload successful")
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    logger::init_logger();

    let routes = Router::new()
        .route("/upload_page", get(upload_page))
        .route("/do_upload", post(do_upload))
        .layer(TraceLayer::new_for_http());

    // 创建 "uploads" 文件夹
    if let Err(err) = tokio::fs::create_dir_all("uploads").await {
        eprintln!("Failed to create 'uploads' directory: {}", err);
        return;
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
