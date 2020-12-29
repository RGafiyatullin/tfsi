use crate::capabilities::Capabilities;

mod real_db;
pub use real_db::RealDb;
pub use real_db::RealDbConfig;

#[derive(Debug)]
pub struct RealCapabilities;

impl Capabilities for RealCapabilities {
    type Db = RealDb;
}
