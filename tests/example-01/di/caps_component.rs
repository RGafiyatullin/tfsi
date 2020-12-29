use ::tfsi::prelude::*;

use crate::capabilities::Db;
use crate::capabilities::HttpServer;

#[derive(Debug)]
pub struct CapsComponent {
    db: Db,
    http_server: HttpServer,
}

impl<'p, 'd> Component<'p, 'd, (Db, HttpServer), ()> for CapsComponent
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, Db, HttpServer, ];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> (Db, HttpServer) {
        (deps.provide(), deps.provide())
    }
}
