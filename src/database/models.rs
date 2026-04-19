use chrono;
use sea_query as sq;
use serde::{Deserialize, Serialize};

/// URL status enum - stored as VARCHAR in database
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UrlStatus {
    Active,
    Inactive,
    Expired,
}

impl std::fmt::Display for UrlStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlStatus::Active => write!(f, "active"),
            UrlStatus::Inactive => write!(f, "inactive"),
            UrlStatus::Expired => write!(f, "expired"),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(sq::Iden)]
pub enum Url {
    #[iden = "urls"]
    Table,
    Id,
    LongUrl,
    ShortUrl,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UrlRow {
    pub id: i64,
    pub long_url: String,
    pub short_url: String,
    pub status: UrlStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
