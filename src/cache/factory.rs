use redis::{Client, aio::ConnectionManager};

pub async fn create_redis_connection(
    redis_url: &str,
) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(redis_url)?;
    ConnectionManager::new(client).await
}
