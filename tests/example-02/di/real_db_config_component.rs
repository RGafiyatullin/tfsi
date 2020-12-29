use crate::app_config::AppConfig;
use crate::capabilities::real_capabilities::RealDbConfig;
use crate::di_prelude::*;

#[derive(Debug, Default)]
pub struct RealDbConfigComponent;

impl<'p, 'd> Component<'p, 'd, &'d RealDbConfig, Self> for RealDbConfigComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d AppConfig];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> &'d RealDbConfig {
        &deps.provide().db
    }
}
