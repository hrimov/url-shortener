use redis::aio::ConnectionManager;
use testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use testcontainers_modules::redis::Redis;

pub async fn setup_test_redis() -> (ContainerAsync<Redis>, ConnectionManager) {
    let container = Redis::default()
        .with_tag("7-alpine")
        .start()
        .await
        .expect("Failed to start redis container");

    let redis_url = format!(
        "redis://127.0.0.1:{}",
        container
            .get_host_port_ipv4(6379)
            .await
            .expect("Failed to get port")
    );

    let conn = url_shortener::cache::create_redis_connection(&redis_url)
        .await
        .expect("Failed to connect to redis");

    (container, conn)
}
