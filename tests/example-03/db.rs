pub trait Db {
    fn query(&self, query: &str);
}
