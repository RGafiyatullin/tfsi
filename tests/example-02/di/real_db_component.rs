use crate::capabilities::real_capabilities::RealDb;
use crate::capabilities::real_capabilities::RealDbConfig;
use crate::di_prelude::*;

#[derive(Debug, Default)]
pub struct RealDbComponent;

impl<'p, 'd> Component<'p, 'd, RealDb, Self> for RealDbComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d RealDbConfig];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> RealDb {
        RealDb::create(deps.provide())
    }
}
