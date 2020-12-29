use ::tfsi::prelude::*;

use crate::config::AppConfig;
use crate::config::DbConfig;
use crate::config::HttpBindConfig;

#[derive(Debug, Default, Clone)]
pub struct ConfigComponent;

impl<'p, 'd> Component<'p, 'd, &'d DbConfig, ()> for ConfigComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d AppConfig];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> &'d DbConfig {
        &deps.provide().db
    }
}

impl<'p, 'd> Component<'p, 'd, &'d HttpBindConfig, ()> for ConfigComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d AppConfig];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> &'d HttpBindConfig {
        &deps.provide().http_bind
    }
}
