use crate::Db;

pub type Requirement<'p, 'd, StaticDb> = Require!['p, 'd, &'d dyn Db, &'d StaticDb];
