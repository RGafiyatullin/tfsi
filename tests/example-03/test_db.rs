use crate::Db;
#[derive(Debug)]
pub struct TestDb;

impl Db for TestDb {
    fn query(&self, query: &str) {
        println!("{:?}::query[{:?}]", self, query);
    }
}
