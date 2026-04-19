use serde::{Deserialize, Serialize};

use crate::database::models::UrlStatus;

#[derive(Serialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct HealthcheckResponse {
    pub status: String,
    pub database: bool,
    pub cache: bool,
}

#[cfg(feature = "swagger")]
#[derive(Serialize, utoipa::ToSchema)]
pub struct GetShortUrlResponse {
    pub long_url: String,
    pub short_url: String,
    pub status: UrlStatus,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CreateShortUrlPayload {
    pub long_url: String,
    pub custom_short_url: Option<String>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CreateShortUrlResponse {
    pub id: i64,
    pub short_url: String,
    pub long_url: String,
    pub status: UrlStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    pub error: String,
}
