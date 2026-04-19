pub async fn create_connection_pool(
    database_url: String,
    max_connections: u32,
    connect_timeout_seconds: u64,
) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(std::time::Duration::from_secs(connect_timeout_seconds))
        .connect(database_url.as_str())
        .await?;
    Ok(pool)
}
