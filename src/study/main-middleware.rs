#![allow(unused)]
use axum::{
    async_trait,
    extract::{extractor_middleware, FromRequest},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use axum_extra::middleware;
use tower_http::trace::TraceLayer;

struct RequireAuth;

#[async_trait]
impl<B> FromRequest<B> for RequireAuth
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut FromRequestParts<B>) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers()
            .and_then(|headers| headers.get(http::header::AUTHORIZATION))
            .and_then(|value| value.to_str().ok());

        if let Some(value) = auth_header {
            if value == "secret" {
                return Ok(Self);
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug,middleware=debug");
    }
    tracing_subscriber::fmt::init();

    let routes = Router::new()
        .route("/foo", get(foo))
        .route("/bar", get(bar))
        .layer(TraceLayer::new_for_http())
        .route_layer(extractor_middleware::<RequireAuth>());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

async fn foo() -> &'static str {
    "Welcome to axum.rs"
}

async fn bar() -> &'static str {
    "Powered by axum.rs"
}
