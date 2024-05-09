#![allow(unused)]

use std::collections::HashMap;

use axum::{
    extract::{Json, Path, Query},
    http::HeaderMap,
    routing::{get, post},
    Form, Router,
};
use axum_extra::TypedHeader;
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/user/:id", get(user_info))
        .route("/user1/:id", get(user_info1))
        .route("/repo/:user/:repo", get(repo_info))
        .route("/repo_struct/:user_name/:repo_name", get(repo_info_struct))
        .route("/subject", get(subject))
        .route("/subject_done", get(subject_opt_done))
        .route("/all", get(all_query))
        .route("/create_user", post(create_user))
        .route("/create_user_ajax", post(create_user_ajax))
        .route("/get_all_headers", get(get_all_headers))
        .route("/get_user_agent", get(get_user_agent))
        .route("/get_user_agent_typed", get(get_user_agent_typed));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

// 通过Path直接解构使用
async fn user_info(Path(id): Path<String>) -> String {
    format!("User info for {}", id)
}

// 没有解构，需要使用id.0获取到参数的值
async fn user_info1(id: Path<String>) -> String {
    format!("User info for {}", id.0)
}

async fn repo_info(Path((user_name, repo_name)): Path<(String, String)>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        user_name, repo_name
    )
}

#[derive(Deserialize)]
pub struct RepoInfo {
    pub user_name: String,
    pub repo_name: String,
}

async fn repo_info_struct(Path(info): Path<RepoInfo>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        info.user_name, info.repo_name
    )
}

#[derive(Deserialize)]
pub struct SubjectArgs {
    pub page: i32,
    pub keyword: String,
}

async fn subject(Query(args): Query<SubjectArgs>) -> String {
    format!("Page {}, keyword: {} of subjects", args.page, args.keyword)
}

#[derive(Deserialize)]
pub struct SubjectArgsOpt {
    pub page: Option<i32>,
    pub keyword: Option<String>,
}

async fn subject_opt_done(Query(args): Query<SubjectArgsOpt>) -> String {
    let page = args.page.unwrap_or(0);
    let keyword = args.keyword.unwrap_or("".to_string());

    format!("Page {}, keyword: {} of subjects", page, keyword)
}

async fn all_query(Query(args): Query<HashMap<String, String>>) -> String {
    format!("{:?}", args)
}

// --------------------------获取表单输入--------------------------------------
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub level: u8,
}

async fn create_user(Form(frm): Form<CreateUser>) -> String {
    format!(
        "Created user: {}, email: {}, level: {}",
        frm.username, frm.email, frm.level
    )
}

async fn create_user_ajax(Json(frm): Json<CreateUser>) -> String {
    format!(
        "Created user: {}, email: {}, level: {}",
        frm.username, frm.email, frm.level
    )
}

async fn get_all_headers(headers: HeaderMap) -> String {
    format!("{:?}", headers)
}

async fn get_user_agent(headers: HeaderMap) -> String {
    headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap()
}

async fn get_user_agent_typed(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
