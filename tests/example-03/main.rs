#![recursion_limit = "16"]

#[macro_use]
extern crate tfsi;

use ::tfsi::prelude::*;

mod db;
pub use db::Db;

mod test_db;
pub use test_db::TestDb;

mod real_db;
pub use real_db::RealDb;

mod requirement;
pub use requirement::Requirement;

#[test]
fn main_run() {
    let db = RealDb;
    let injector = Injector::create()
        .with_value(&db as &dyn Db)
        .with_value(&db);
    run::<RealDb, _, _>(&injector);
}

#[test]
fn main_test() {
    let db = TestDb;
    let injector = Injector::create()
        .with_value(&db as &dyn Db)
        .with_value(&db);
    run::<TestDb, _, _>(&injector);
}

fn run<
    'p,
    'd: 'p,
    StaticDb: Db + 'd,
    I: 'p + FitRequirement<'p, 'd, Requirement<'p, 'd, StaticDb>, Idx>,
    Idx,
>(
    injector: &'p I,
) {
    let provider = Requirement::require(injector);
    let dyn_db: &dyn Db = provider.provide();
    let static_db: &StaticDb = provider.provide();
    dyn_db.query("SELECT 'dyn' FROM dual");
    static_db.query("SELECT 'static' FROM dual");
}
