use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use metrics_exporter_prometheus::PrometheusHandle;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use super::{handlers, middleware};
use crate::metrics;

#[cfg(feature = "swagger")]
use {super::schemas, crate::database::models::UrlStatus, utoipa::OpenApi};

#[cfg(feature = "swagger")]
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::healthcheck,
        handlers::get_short_url,
        handlers::create_short_url,
    ),
    components(
        schemas(
            schemas::HealthcheckResponse,
            schemas::GetShortUrlResponse,
            schemas::CreateShortUrlPayload,
            schemas::CreateShortUrlResponse,
            schemas::ErrorResponse,
            UrlStatus,
        )
    ),
    tags(
        (name = "url-shortener", description = "URL shortener API endpoints")
    ),
    info(
        title = "URL Shortener API",
        version = "0.1.0",
        description = "A simple URL shortener service",
    )
)]
struct ApiDoc;

pub fn create_router(state: handlers::AppState, metrics_handle: PrometheusHandle) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let metrics_router = Router::new()
        .route("/metrics", get(metrics::render))
        .with_state(metrics_handle);

    let api_router = Router::new()
        .route("/healthcheck", get(handlers::healthcheck))
        .route("/url", post(handlers::create_short_url))
        .route("/{short_url}", get(handlers::get_short_url))
        .with_state(state);

    let router = Router::new();

    #[cfg(feature = "swagger")]
    let router = {
        let swagger_router =
            utoipa_swagger_ui::SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi());
        router.merge(swagger_router)
    };

    router
        .merge(metrics_router)
        .merge(api_router)
        .layer(from_fn(middleware::track_request))
        .layer(cors_layer)
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

pub async fn create_listener(bind_string: String) -> std::io::Result<TcpListener> {
    TcpListener::bind(bind_string).await
}
