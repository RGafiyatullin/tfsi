use ::tfsi::prelude::*;

use crate::capabilities::Db;
use crate::config::DbConfig;

#[derive(Debug, Default, Clone)]
pub struct DbComponent;

impl<'p, 'd> Component<'p, 'd, Db, ()> for DbComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d DbConfig];

    fn component_provide(&'p self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Db {
        Db::create(deps.provide())
    }
}
