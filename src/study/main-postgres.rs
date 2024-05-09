#![allow(unused)]

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Json, Router,
};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value as JsonValue};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Pool, Postgres, Row,
};
use std::{env, fmt::Display, str::FromStr, sync::Arc};
use tower_http::trace::TraceLayer;

mod logger;

pub struct AppState {
    db: Pool<Postgres>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub balance: i32,
}
#[derive(Deserialize)]
pub struct CreateAccount {
    pub username: String,
    pub balance: i32,
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

// 所有账户
async fn list(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<JsonValue>)> {
    let notes = match sqlx::query_as!(
        Account,
        "SELECT id, username, balance FROM account ORDER BY id DESC",
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(result) => result,
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": "Something bad happened while fetching all note items",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "data": notes
    });

    Ok(Json(json_response))
}

// 插入数据
async fn insert_lists(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateAccount>,
) -> Result<impl IntoResponse, (StatusCode, Json<JsonValue>)> {
    let query_result =
        sqlx::query_as!(Account,
        "INSERT INTO account (username, balance) VALUES ($1, $2) RETURNING id, username, balance",
        body.username,
        body.balance,
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(row) => {
            let note_response = json!({"status": "success","data": json!({
                "id": row.id,
                "username": row.username,
                "balance": row.balance,
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

async fn find(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(
        "SELECT id,username,balance FROM account WHERE id=$1 ORDER BY id DESC LIMIT 1",
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(row) => {
            let note_response = json!({"status": "success","data": json!({
                "id": row.id,
                "username": row.username,
                "balance": row.balance,
            })});

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

async fn update(
    Path((id, balance)): Path<(i32, i32)>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(
        "update account set balance=$1 where id=$2 returning id, username, balance",
        balance,
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "id": note.id,
                "username": note.username,
                "balance": note.balance,
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

//
async fn delete(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM account WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });

        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let success_response = json!({
        "status": "success",
        "message": format!("Deleted id = {} record", id)
    });

    Ok(Json(success_response))
}

async fn transfer(
    Path((from_id, to_id, balance)): Path<(i32, i32, i32)>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // 修改出账记录
    let stmt_c = sqlx::query!(
        "UPDATE account SET balance=balance-$1 WHERE id=$2 AND balance>=$1 returning id, username, balance",
        balance,
        from_id
    )
    .fetch_one(&data.db)
    .await;

    // 修改入账记录
    let stmt_r = sqlx::query!(
        "UPDATE account SET balance=balance+$1 WHERE id=$2 returning id, username, balance",
        balance,
        to_id
    )
    .fetch_one(&data.db)
    .await;

    if let (Ok(stmt_c), Ok(stmt_r)) = (stmt_c, stmt_r) {
        let success_response = json!({
            "status": "success",
            "message": "转账成功",
            "data":json!({
                    "from":{
                        "id": stmt_c.id,
                        "username": stmt_c.username,
                        "balance": stmt_c.balance
                    },
                    "to":{
                        "id": stmt_r.id,
                        "username": stmt_r.username,
                        "balance": stmt_r.balance
                    },
            })
        });

        return Ok(Json(success_response));
    } else {
        let error_response = json!({
            "status": "fail",
            "message": "出账或入账记录更新失败！"
        });

        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    logger::init_logger();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let app_state = Arc::new(AppState { db: pool.clone() });

    let routes = Router::new()
        .route("/check", get(health_checker_handler))
        .route("/", get(list))
        .route("/insert", post(insert_lists))
        .route("/find/:id", get(find))
        .route("/update/:id/:balance", get(update))
        .route("/delete/:id", get(delete))
        .route("/transfer/:from_id/:to_id/:balance", get(transfer))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
