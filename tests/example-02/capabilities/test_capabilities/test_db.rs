use crate::capabilities::Db;

#[derive(Debug, Clone, Default)]
pub struct TestDb {}

impl Db for TestDb {}
