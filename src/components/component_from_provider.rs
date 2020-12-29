use crate::component::Component;
use crate::provide::Provide;
use crate::require::Require;
use crate::requirements::RequireNil;

/// A `Component` constructed from a `Provide`.
#[derive(Debug, Clone)]
pub struct ComponentFromProvider<P>(P);

impl<P> From<P> for ComponentFromProvider<P> {
    fn from(p: P) -> Self {
        Self(p)
    }
}

impl<'p, 'd, P, T, I> Component<'p, 'd, T, I> for ComponentFromProvider<P>
where
    P: Provide<'p, 'd, T, I>,
    T: 'd,
{
    type Requirement = RequireNil;

    fn component_provide(&'p self, _deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> T {
        self.0.provide()
    }
}
