use crate::capabilities::Capabilities;
use crate::di_prelude::*;

pub struct App<Caps>
where
    Caps: Capabilities,
{
    db: Caps::Db,
}

impl<Caps> App<Caps>
where
    Caps: Capabilities,
{
    pub fn run(self) {
        println!("Running App");
        println!("- db: {:?}", self.db);
    }
}

#[derive(Debug)]
pub struct AppComponent<Caps>(std::marker::PhantomData<Caps>);
impl<Caps> Default for AppComponent<Caps> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'p, 'd, Caps> Component<'p, 'd, App<Caps>, Self> for AppComponent<Caps>
where
    Caps: Capabilities,
    'd: 'p,
    Caps: 'd,
{
    type Requirement = Require!['p, 'd, Caps::Db];

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> App<Caps> {
        App { db: deps.provide() }
    }
}
