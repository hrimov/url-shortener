use redis::{AsyncCommands, aio::ConnectionManager};
use serde::{Deserialize, Serialize};

use crate::database::models::{UrlRow, UrlStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedUrl {
    pub id: i64,
    pub long_url: String,
    pub status: UrlStatus,
}

impl CachedUrl {
    /// Convert back into a UrlRow (timestamps are zeroed, they are not needed for redirects).
    pub fn into_url_row(self, short_url: &str) -> UrlRow {
        UrlRow {
            id: self.id,
            long_url: self.long_url,
            short_url: short_url.to_string(),
            status: self.status,
            created_at: chrono::NaiveDateTime::default(),
            updated_at: chrono::NaiveDateTime::default(),
        }
    }
}

impl From<&UrlRow> for CachedUrl {
    fn from(row: &UrlRow) -> Self {
        CachedUrl {
            id: row.id,
            long_url: row.long_url.clone(),
            status: row.status.clone(),
        }
    }
}

pub struct CacheGateway {
    conn: ConnectionManager,
}

impl CacheGateway {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    pub async fn get_url(
        &mut self,
        short_url: &str,
    ) -> Result<Option<CachedUrl>, redis::RedisError> {
        let key = format!("url:{}", short_url);
        let cached: Option<String> = self.conn.get(&key).await?;

        match cached {
            Some(json) => {
                let url: CachedUrl = serde_json::from_str(&json).map_err(|e| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "JSON deserialization error",
                        e.to_string(),
                    ))
                })?;
                Ok(Some(url))
            }
            None => Ok(None),
        }
    }

    pub async fn set_url(
        &mut self,
        short_url: &str,
        url_row: &UrlRow,
        ttl_seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let key = format!("url:{}", short_url);
        let cached_url = CachedUrl::from(url_row);
        let json = serde_json::to_string(&cached_url).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "JSON serialization error",
                e.to_string(),
            ))
        })?;

        let _: () = self.conn.set_ex(&key, json, ttl_seconds).await?;
        Ok(())
    }
}
