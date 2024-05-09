#![allow(unused)]
use axum::handler::HandlerWithoutStateExt;
use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug,middleware=debug");
    }
    tracing_subscriber::fmt::init();

    let routes = Router::new()
        .route("/static/axum-rs.txt", get(axum_rs_txt))
        .nest_service(
            "/static",
            ServeDir::new("static").not_found_service(handle_404.into_service()),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

async fn axum_rs_txt() -> String {
    std::fs::read_to_string("static/axum-rs.txt").unwrap()
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
