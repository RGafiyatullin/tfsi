use crate::component::Component;
use crate::require::Require;

#[derive(Debug)]
pub struct IdxLeft<I>(std::marker::PhantomData<I>);
#[derive(Debug)]
pub struct IdxRight<I>(std::marker::PhantomData<I>);

/// A `Component` that combines two `Component`s.
///
/// It forwards the `Component::component_provide` methods to the underlying `Component`s.
#[derive(Debug, Clone)]
pub struct ComponentPair<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> ComponentPair<Left, Right> {
    /// Consume this `ComponentPair` and yield its `left` and `right`.
    pub fn into_inner(self) -> (Left, Right) {
        (self.left, self.right)
    }
}

impl<Left, Right> From<(Left, Right)> for ComponentPair<Left, Right> {
    fn from(pair: (Left, Right)) -> Self {
        let (left, right) = pair;
        Self { left, right }
    }
}

impl<'p, 'd, T, I, Left, Right> Component<'p, 'd, T, IdxLeft<I>> for ComponentPair<Left, Right>
where
    Left: Component<'p, 'd, T, I>,
{
    type Requirement = Left::Requirement;

    fn component_provide(&'p self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> T {
        self.left.component_provide(deps)
    }
}

impl<'p, 'd, T, I, Left, Right> Component<'p, 'd, T, IdxRight<I>> for ComponentPair<Left, Right>
where
    Right: Component<'p, 'd, T, I>,
{
    type Requirement = Right::Requirement;

    fn component_provide(&'p self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> T {
        self.right.component_provide(deps)
    }
}
