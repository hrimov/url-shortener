#[path = "common/postgres.rs"]
mod postgres_common;

use postgres_common::setup_test_db;
use url_shortener::services::UrlService;
use url_shortener::services::url_service::UrlServiceError;

#[tokio::test]
async fn test_create_short_url_success() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    let result = service
        .create_short_url("https://example.com".to_string(), None)
        .await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert_eq!(url_row.long_url, "https://example.com");
    assert_eq!(url_row.short_url.len(), 7);
}

#[tokio::test]
async fn test_create_short_url_with_custom() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    let result = service
        .create_short_url(
            "https://example.com".to_string(),
            Some("custom1".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert_eq!(url_row.short_url, "custom1");
}

#[tokio::test]
async fn test_create_short_url_empty_long_url() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    let result = service.create_short_url("".to_string(), None).await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UrlServiceError::EmptyLongUrl));
}

#[tokio::test]
async fn test_create_short_url_idempotent() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    let first = service
        .create_short_url(
            "https://example.com".to_string(),
            Some("test123".to_string()),
        )
        .await
        .unwrap();

    let second = service
        .create_short_url(
            "https://example.com".to_string(),
            Some("test123".to_string()),
        )
        .await
        .unwrap();

    assert_eq!(first.id, second.id);
    assert_eq!(first.short_url, second.short_url);
    assert_eq!(first.long_url, second.long_url);
}

#[tokio::test]
async fn test_create_short_url_custom_conflict() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    service
        .create_short_url(
            "https://example.com".to_string(),
            Some("taken1".to_string()),
        )
        .await
        .unwrap();

    let result = service
        .create_short_url("https://other.com".to_string(), Some("taken1".to_string()))
        .await;

    assert!(matches!(
        result.unwrap_err(),
        UrlServiceError::CustomShortUrlConflict
    ));
}

#[tokio::test]
async fn test_create_short_url_invalid_custom() {
    let (_container, pool) = setup_test_db().await;
    let service = UrlService::new(&pool, None, 3600);

    let result = service
        .create_short_url(
            "https://example.com".to_string(),
            Some("hello world".to_string()),
        )
        .await;
    assert!(matches!(
        result.unwrap_err(),
        UrlServiceError::InvalidCustomShortUrl
    ));

    let result = service
        .create_short_url("https://example.com".to_string(), Some("".to_string()))
        .await;
    assert!(matches!(
        result.unwrap_err(),
        UrlServiceError::InvalidCustomShortUrl
    ));
}

#[tokio::test]
async fn test_get_url_by_short_url_success() {
    let (_container, pool) = setup_test_db().await;
    let mut service = UrlService::new(&pool, None, 3600);

    let created = service
        .create_short_url("https://example.com".to_string(), None)
        .await
        .unwrap();

    let result = service.get_url_by_short_url(&created.short_url).await;

    assert!(result.is_ok());
    let url_row = result.unwrap();
    assert_eq!(url_row.long_url, "https://example.com");
}

#[tokio::test]
async fn test_get_url_by_short_url_not_found() {
    let (_container, pool) = setup_test_db().await;
    let mut service = UrlService::new(&pool, None, 3600);

    let result = service.get_url_by_short_url("nonexistent").await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), UrlServiceError::NotFound));
}
