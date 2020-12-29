use crate::config::DbConfig;

#[derive(Debug)]
pub struct Db {
    config: DbConfig,
}

impl Db {
    pub fn create(config: &DbConfig) -> Self {
        Self {
            config: config.to_owned(),
        }
    }
}
