use redis::AsyncCommands;
use redis::Client;
use std::error::Error;

const REDIS_DSN: &str = "redis://127.0.0.1:6379/";

// TODO 获取 redis 连接
pub async fn connect_to_redis() -> Result<redis::aio::MultiplexedConnection, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| err.to_string())?;
    Ok(conn)
}

// TODO 写入Redis
pub async fn write_to_redis<T>(key: &String, value: T) -> Result<(), String>
where
    T: ToString,
{
    let mut conn = connect_to_redis().await?;
    conn.set(key, value.to_string())
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}

// 删除 redis 缓存的值
pub async fn delete_from_redis(key: &String) -> Result<(), String> {
    let mut conn = connect_to_redis().await?;
    let result: i32 = conn.del(key).await.map_err(|err| err.to_string())?;
    if result == 1 {
        Ok(())
    } else {
        Err(format!("Failed to delete key: {}", key))
    }
}

// TODO 获取Redis
pub async fn read_from_redis<T>(key: &String) -> Result<T, String>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut conn = connect_to_redis().await?;
    let value_str: Option<String> = conn.get(key).await.map_err(|err| err.to_string())?;
    match value_str {
        Some(value) => {
            let parsed_value: T = value
                .parse()
                .map_err(|err| format!("Failed to parse value: {}", err))?;
            Ok(parsed_value)
        }
        None => Err(format!("Key '{}' not found in Redis", key)),
    }
}

// TODO 设置自动过期
pub async fn write_ex_to_redis<T>(key: String, value: T, expire_seconds: u64) -> Result<(), String>
where
    T: ToString,
{
    let mut conn = connect_to_redis().await?;
    conn.set_ex(key, value.to_string(), expire_seconds)
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
