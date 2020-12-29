use super::DbConfig;
use super::HttpBindConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub http_bind: HttpBindConfig,
    pub db: DbConfig,
}
