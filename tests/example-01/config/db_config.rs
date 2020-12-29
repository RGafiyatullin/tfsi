#[derive(Debug, Clone)]
pub struct DbConfig {
    pub pool_size: usize,
    pub db_uri: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            pool_size: 1,
            db_uri: "mysql://user:password@localhost:3306/helloes".to_owned(),
        }
    }
}
