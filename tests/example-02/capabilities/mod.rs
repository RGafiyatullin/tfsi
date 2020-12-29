pub mod real_capabilities;
pub mod test_capabilities;

mod db;
pub use db::Db;

pub trait Capabilities {
    type Db: Db;
}
