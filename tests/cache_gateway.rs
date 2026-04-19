#[path = "common/redis.rs"]
mod redis_common;

use redis_common::setup_test_redis;
use url_shortener::cache::CacheGateway;
use url_shortener::database::models::{UrlRow, UrlStatus};

fn create_test_url_row() -> UrlRow {
    UrlRow {
        id: 1,
        long_url: "https://example.com".to_string(),
        short_url: "abc123".to_string(),
        status: UrlStatus::Active,
        created_at: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
        updated_at: chrono::DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
    }
}

#[tokio::test]
async fn test_set_and_get_url() {
    let (_container, conn) = setup_test_redis().await;
    let mut gateway = CacheGateway::new(conn);

    let url_row = create_test_url_row();

    gateway
        .set_url(&url_row.short_url, &url_row, 60)
        .await
        .expect("Failed to set URL");

    let cached = gateway
        .get_url(&url_row.short_url)
        .await
        .expect("Failed to get URL");

    assert!(cached.is_some());
    let cached_url = cached.unwrap();
    assert_eq!(cached_url.long_url, url_row.long_url);
    assert_eq!(cached_url.id, url_row.id);
    assert_eq!(cached_url.status, UrlStatus::Active);
}

#[tokio::test]
async fn test_get_nonexistent_url() {
    let (_container, conn) = setup_test_redis().await;
    let mut gateway = CacheGateway::new(conn);

    let cached = gateway
        .get_url("nonexistent")
        .await
        .expect("Failed to get URL");

    assert!(cached.is_none());
}

#[tokio::test]
async fn test_cache_expiration() {
    let (_container, conn) = setup_test_redis().await;
    let mut gateway = CacheGateway::new(conn);

    let url_row = create_test_url_row();

    gateway
        .set_url(&url_row.short_url, &url_row, 1)
        .await
        .expect("Failed to set URL");

    let cached = gateway.get_url(&url_row.short_url).await.unwrap();
    assert!(cached.is_some());

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let cached = gateway.get_url(&url_row.short_url).await.unwrap();
    assert!(cached.is_none());
}
