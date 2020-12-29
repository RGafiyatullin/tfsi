use crate::Db;

#[derive(Debug)]
pub struct RealDb;

impl Db for RealDb {
    fn query(&self, query: &str) {
        println!("{:?}::query[{:?}]", self, query);
    }
}
