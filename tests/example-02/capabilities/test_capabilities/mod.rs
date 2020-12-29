use crate::capabilities::Capabilities;

mod test_db;
pub use test_db::TestDb;

#[derive(Debug)]
pub struct TestCapabilities;

impl Capabilities for TestCapabilities {
    type Db = TestDb;
}
