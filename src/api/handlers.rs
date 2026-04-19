use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use super::schemas::{
    CreateShortUrlPayload, CreateShortUrlResponse, ErrorResponse, HealthcheckResponse,
};
use crate::services::{UrlService, url_service::UrlServiceError};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub cache_conn: Option<ConnectionManager>,
    pub cache_ttl_seconds: u64,
}

#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = 200, description = "Service is healthy", body = HealthcheckResponse),
        (status = 503, description = "Service is unhealthy", body = ErrorResponse)
    )
))]
pub async fn healthcheck(State(state): State<AppState>) -> Response {
    if let Err(err) = sqlx::query("SELECT 1").execute(&state.db_pool).await {
        tracing::error!("Healthcheck failed: database unreachable: {}", err);
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database unreachable".to_string(),
            }),
        )
            .into_response();
    }

    let redis_ok = if let Some(mut conn) = state.cache_conn.clone() {
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .is_ok()
    } else {
        false
    };

    Json(HealthcheckResponse {
        status: "ok".to_string(),
        database: true,
        cache: redis_ok,
    })
    .into_response()
}

#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/{short_url}",
    params(
        ("short_url" = String, Path, description = "Short URL identifier")
    ),
    responses(
        (status = 307, description = "Temporary redirect to original URL"),
        (status = 404, description = "Short URL not found", body = ErrorResponse),
        (status = 410, description = "Short URL is inactive or expired", body = ErrorResponse),
        (status = 500, description = "Database error", body = ErrorResponse)
    )
))]
pub async fn get_short_url(
    State(state): State<AppState>,
    Path(short_url): Path<String>,
) -> Response {
    let mut service = UrlService::new(
        &state.db_pool,
        state.cache_conn.clone(),
        state.cache_ttl_seconds,
    );

    match service.get_url_by_short_url(&short_url).await {
        Ok(url_row) => Redirect::temporary(&url_row.long_url).into_response(),
        Err(UrlServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Short URL not found".to_string(),
            }),
        )
            .into_response(),
        Err(UrlServiceError::Inactive) => (
            StatusCode::GONE,
            Json(ErrorResponse {
                error: "This short URL is inactive or expired".to_string(),
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}

#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/url",
    request_body = CreateShortUrlPayload,
    responses(
        (status = 201, description = "Short URL created successfully", body = CreateShortUrlResponse),
        (status = 400, description = "Invalid request payload", body = ErrorResponse),
        (status = 409, description = "Short URL already exists", body = ErrorResponse),
        (status = 500, description = "Database error", body = ErrorResponse)
    )
))]
pub async fn create_short_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateShortUrlPayload>,
) -> Response {
    let service = UrlService::new(
        &state.db_pool,
        state.cache_conn.clone(),
        state.cache_ttl_seconds,
    );

    match service
        .create_short_url(payload.long_url, payload.custom_short_url)
        .await
    {
        Ok(url_row) => (
            StatusCode::CREATED,
            Json(CreateShortUrlResponse {
                id: url_row.id,
                short_url: url_row.short_url,
                long_url: url_row.long_url,
                status: url_row.status,
                created_at: url_row.created_at.and_utc().to_rfc3339(),
                updated_at: url_row.updated_at.and_utc().to_rfc3339(),
            }),
        )
            .into_response(),
        Err(UrlServiceError::EmptyLongUrl)
        | Err(UrlServiceError::InvalidUrl)
        | Err(UrlServiceError::InvalidCustomShortUrl) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid request payload".to_string(),
            }),
        )
            .into_response(),
        Err(UrlServiceError::CustomShortUrlConflict) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Custom short URL already exists".to_string(),
            }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}
