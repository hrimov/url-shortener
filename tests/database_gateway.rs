#[path = "common/postgres.rs"]
mod postgres_common;

use postgres_common::setup_test_db;
use url_shortener::database::{UrlGateway, models::UrlStatus};

#[tokio::test]
async fn test_create_url() {
    let (_container, pool) = setup_test_db().await;
    let gateway = UrlGateway::new(&pool);

    let result = gateway.create_url("https://example.com", "abc123").await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert_eq!(url_row.long_url, "https://example.com");
    assert_eq!(url_row.short_url, "abc123");
    assert_eq!(url_row.status, UrlStatus::Active);
}

#[tokio::test]
async fn test_create_duplicate_short_url_fails() {
    let (_container, pool) = setup_test_db().await;
    let gateway = UrlGateway::new(&pool);

    gateway
        .create_url("https://example.com", "abc123")
        .await
        .expect("First insert should succeed");

    let result = gateway.create_url("https://another.com", "abc123").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_url_by_short_url() {
    let (_container, pool) = setup_test_db().await;
    let gateway = UrlGateway::new(&pool);

    gateway
        .create_url("https://example.com", "abc123")
        .await
        .expect("Insert should succeed");

    let result = gateway.get_url_by_short_url("abc123").await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert!(url_row.is_some());
    let url_row = url_row.unwrap();
    assert_eq!(url_row.long_url, "https://example.com");
    assert_eq!(url_row.short_url, "abc123");
}

#[tokio::test]
async fn test_get_url_by_short_url_not_found() {
    let (_container, pool) = setup_test_db().await;
    let gateway = UrlGateway::new(&pool);

    let result = gateway.get_url_by_short_url("nonexistent").await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert!(url_row.is_none());
}
