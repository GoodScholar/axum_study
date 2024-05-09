#![allow(unused)]

use askama::Template;
use axum::{response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
mod logger;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
    pub text: String,
}

async fn index() -> Result<Html<String>, String> {
    let name = String::from("axum中文网");
    let text = String::from("test.html");

    let tpl = IndexTemplate { name, text };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    logger::init_logger();

    let routes = Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
