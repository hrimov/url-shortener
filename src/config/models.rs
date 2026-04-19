use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub application: ApplicationConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl ApplicationConfig {
    pub fn bind_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String,

    pub max_connections: u32,
    pub connect_timeout_seconds: u64,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database_name,
        )
    }
}

#[derive(Deserialize)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16,
    pub ttl_seconds: u64,
}

impl CacheConfig {
    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_bind_string() {
        let app_config = ApplicationConfig {
            host: "localhost".to_string(),
            port: 1234,
            log_level: "info".to_string(),
        };
        assert_eq!(app_config.bind_string(), "localhost:1234");
    }

    #[test]
    fn test_database_config_connection_string() {
        let database_config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            user: "user".to_string(),
            password: "password".to_string(),
            database_name: "database".to_string(),

            max_connections: 5,
            connect_timeout_seconds: 10,
        };

        assert_eq!(
            database_config.connection_string(),
            "postgres://user:password@localhost:5432/database",
        );
    }
}
