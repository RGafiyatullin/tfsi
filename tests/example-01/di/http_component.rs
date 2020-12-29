use ::tfsi::prelude::*;

use crate::capabilities::HttpServer;
use crate::config::HttpBindConfig;

#[derive(Debug, Default, Clone)]
pub struct HttpComponent;

impl<'p, 'd> Component<'p, 'd, HttpServer, ()> for HttpComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, &'d HttpBindConfig];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> HttpServer {
        HttpServer::create(deps.provide())
    }
}
