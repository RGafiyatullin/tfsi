use crate::capabilities::Db;

#[derive(Debug, Clone)]
pub struct RealDbConfig {}

#[derive(Debug)]
pub struct RealDb {
    config: RealDbConfig,
}

impl RealDb {
    pub fn create(config: &RealDbConfig) -> Self {
        Self {
            config: config.to_owned(),
        }
    }
}

impl Db for RealDb {}
