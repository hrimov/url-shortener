mod api;
mod cache;
mod config;
mod database;
mod encoding;
mod metrics;
mod services;

use std::env;

use crate::api::{create_listener, create_router, handlers::AppState};
use crate::cache::create_redis_connection;
use crate::config::load_config;
use crate::database::{create_connection_pool, migrations};

const DEFAULT_CONFIG_PATH: &str = "config/template.toml";

#[tokio::main]
async fn main() {
    let env_config_path = env::var("CONFIG_PATH");
    let config_path = env_config_path.unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string());
    let config = match load_config(&config_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{}. Set the CONFIG_PATH env variable", err,);
            std::process::exit(2);
        }
    };

    let log_format = env::var("LOG_FORMAT").unwrap_or_default();

    if log_format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new(&config.application.log_level)
                }),
            )
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new(&config.application.log_level)
                }),
            )
            .init();
    }

    let listener = match create_listener(config.application.bind_string()).await {
        Ok(l) => l,
        Err(err) => {
            tracing::error!("Listener creation error: {}", err);
            std::process::exit(3);
        }
    };

    tracing::info!(
        "Connecting to database with {} timeout seconds",
        config.database.connect_timeout_seconds
    );

    let postgres_pool = match create_connection_pool(
        config.database.connection_string(),
        config.database.max_connections,
        config.database.connect_timeout_seconds,
    )
    .await
    {
        Ok(p) => p,
        Err(err) => {
            tracing::error!("Database connection error: {}", err);
            std::process::exit(4);
        }
    };

    if env::var("RUN_MIGRATIONS_ONLY").is_ok() {
        match migrations::run_migrations(&config.database.connection_string()).await {
            Ok(_) => {
                tracing::info!("Migrations completed successfully");
                return;
            }
            Err(err) => {
                tracing::error!("Failed to run database migrations: {}", err);
                std::process::exit(5);
            }
        }
    }

    tracing::info!("Connecting to Redis at {}", config.cache.redis_url());
    let redis_conn = match create_redis_connection(&config.cache.redis_url()).await {
        Ok(conn) => {
            tracing::info!("Successfully connected to Redis");
            Some(conn)
        }
        Err(err) => {
            tracing::warn!(
                "Failed to connect to Redis: {}. Continuing without cache.",
                err
            );
            None
        }
    };

    let app_state = AppState {
        db_pool: postgres_pool,
        cache_conn: redis_conn,
        cache_ttl_seconds: config.cache.ttl_seconds,
    };

    let metrics_handle = metrics::install_recorder();
    let router = create_router(app_state, metrics_handle);

    tracing::info!("Starting server on {}", config.application.bind_string());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("Server shut down gracefully");
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to register SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => tracing::info!("Received SIGINT, starting graceful shutdown"),
            _ = sigterm.recv() => tracing::info!("Received SIGTERM, starting graceful shutdown"),
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await.ok();
        tracing::info!("Received SIGINT, starting graceful shutdown");
    }
}
