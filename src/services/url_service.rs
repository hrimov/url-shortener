use redis::aio::ConnectionManager;
use regex::Regex;
use sqlx::PgPool;
use std::sync::LazyLock;

use crate::cache::CacheGateway;
use crate::database::{UrlGateway, models::UrlRow};

pub struct UrlService<'a> {
    db_gateway: UrlGateway<'a>,
    cache_gateway: Option<CacheGateway>,
    cache_ttl_seconds: u64,
}

const MAX_COLLISION_RETRIES: u32 = 5;

#[derive(Debug, thiserror::Error)]
pub enum UrlServiceError {
    #[error("long_url cannot be empty")]
    EmptyLongUrl,
    #[error("Invalid URL: only http and https schemes are allowed")]
    InvalidUrl,
    #[error("custom_short_url must be between 1 and 50 alphanumeric characters")]
    InvalidCustomShortUrl,
    #[error("Custom short URL already exists")]
    CustomShortUrlConflict,
    #[error("Short URL not found")]
    NotFound,
    #[error("Short URL is inactive or expired")]
    Inactive,
    #[error("Could not generate unique short URL after {MAX_COLLISION_RETRIES} attempts")]
    CollisionRetryExceeded,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> UrlService<'a> {
    pub fn new(
        pool: &'a PgPool,
        cache_conn: Option<ConnectionManager>,
        cache_ttl_seconds: u64,
    ) -> Self {
        Self {
            db_gateway: UrlGateway::new(pool),
            cache_gateway: cache_conn.map(CacheGateway::new),
            cache_ttl_seconds,
        }
    }

    pub async fn create_short_url(
        &self,
        long_url: String,
        custom_short_url: Option<String>,
    ) -> Result<UrlRow, UrlServiceError> {
        if long_url.is_empty() {
            return Err(UrlServiceError::EmptyLongUrl);
        }

        validate_url(&long_url)?;

        match custom_short_url {
            Some(custom) => self.create_with_custom(&long_url, &custom).await,
            None => self.create_with_generated_code(&long_url).await,
        }
    }

    async fn create_with_custom(
        &self,
        long_url: &str,
        custom: &str,
    ) -> Result<UrlRow, UrlServiceError> {
        if custom.is_empty()
            || custom.len() > 50
            || !custom.bytes().all(|b| b.is_ascii_alphanumeric())
        {
            return Err(UrlServiceError::InvalidCustomShortUrl);
        }

        match self.db_gateway.create_url(long_url, custom).await {
            Ok(row) => Ok(row),
            Err(err) if is_unique_violation(&err) => {
                if let Some(existing) = self.db_gateway.get_url_by_short_url(custom).await?
                    && existing.long_url == long_url
                {
                    return Ok(existing);
                }
                Err(UrlServiceError::CustomShortUrlConflict)
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn create_with_generated_code(&self, long_url: &str) -> Result<UrlRow, UrlServiceError> {
        for attempt in 0..MAX_COLLISION_RETRIES {
            let candidate = crate::encoding::generate(long_url, attempt);

            match self.db_gateway.create_url(long_url, &candidate).await {
                Ok(row) => return Ok(row),
                Err(err) if is_unique_violation(&err) => {
                    if let Some(existing) = self.db_gateway.get_url_by_short_url(&candidate).await?
                        && existing.long_url == long_url
                    {
                        return Ok(existing);
                    }
                    tracing::warn!(
                        "Hash collision on attempt {} for short_url={}",
                        attempt,
                        candidate
                    );
                    continue;
                }
                Err(err) => return Err(err.into()),
            }
        }
        Err(UrlServiceError::CollisionRetryExceeded)
    }

    /// Get a URL by its short identifier (with caching)
    pub async fn get_url_by_short_url(
        &mut self,
        short_url: &str,
    ) -> Result<UrlRow, UrlServiceError> {
        if let Some(cache_gateway) = &mut self.cache_gateway {
            match cache_gateway.get_url(short_url).await {
                Ok(Some(cached)) => {
                    tracing::debug!("Cache hit for short_url: {}", short_url);
                    metrics::counter!("cache_operations_total", "result" => "hit").increment(1);
                    return Ok(cached.into_url_row(short_url));
                }
                Ok(None) => {
                    tracing::debug!("Cache miss for short_url: {}", short_url);
                    metrics::counter!("cache_operations_total", "result" => "miss").increment(1);
                }
                Err(err) => {
                    tracing::warn!("Cache error for short_url: {}, error: {}", short_url, err);
                }
            }
        }

        let url_row = self
            .db_gateway
            .get_url_by_short_url(short_url)
            .await?
            .ok_or(UrlServiceError::NotFound)?;

        if url_row.status != crate::database::models::UrlStatus::Active {
            return Err(UrlServiceError::Inactive);
        }

        if let Some(cache_gateway) = &mut self.cache_gateway
            && let Err(err) = cache_gateway
                .set_url(short_url, &url_row, self.cache_ttl_seconds)
                .await
        {
            tracing::warn!("Failed to cache URL {}: {}", short_url, err);
        }

        Ok(url_row)
    }
}

static HOSTNAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,63}$")
        .expect("invalid hostname regex")
});

fn validate_url(input: &str) -> Result<(), UrlServiceError> {
    let parsed = url::Url::parse(input).map_err(|_| UrlServiceError::InvalidUrl)?;
    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(UrlServiceError::InvalidUrl),
    }
    let host = parsed.host_str().ok_or(UrlServiceError::InvalidUrl)?;
    if !HOSTNAME_RE.is_match(host) {
        return Err(UrlServiceError::InvalidUrl);
    }
    Ok(())
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        return db_err.code().is_some_and(|code| code == "23505");
    }
    false
}
