use crate::component::Component;
use crate::require::Require;

/// A `Component` over some other `Component`.
///
/// Can be used to share `Component`s.
#[derive(Debug)]
pub struct ComponentForward<'p, SharedComponent>(&'p SharedComponent);

impl<'p, C> From<&'p C> for ComponentForward<'p, C> {
    fn from(c: &'p C) -> Self {
        Self(c)
    }
}

/// An Index to access `ComponentForward`
#[derive(Debug)]
pub struct IdxComponentForward<I>(std::marker::PhantomData<I>);

impl<'p, C> Clone for ComponentForward<'p, C> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'p, 'd, C, Output, I> Component<'p, 'd, Output, IdxComponentForward<I>>
    for ComponentForward<'p, C>
where
    Output: 'd,
    C: Component<'p, 'd, Output, I>,
{
    type Requirement = C::Requirement;

    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> Output {
        self.0.component_provide(deps)
    }
}
