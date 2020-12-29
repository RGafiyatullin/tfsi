use crate::provide::Provide;

/// A `Provide` that combines two `Provide`s.
///
/// It forwards the `Provide::provide` methods to the underlying `Provide`s.
#[derive(Debug, Clone)]
pub struct ProvidePair<L, R> {
    left: L,
    right: R,
}
impl<L, R> ProvidePair<L, R> {
    /// Consume this `ProvidePair` and yield its `left` and `right`.
    pub fn into_inner(self) -> (L, R) {
        (self.left, self.right)
    }
}

/// Index to access the left side of `ProvidePair`
#[derive(Debug)]
pub struct IdxLeft<I>(std::marker::PhantomData<I>);

/// Index to access the right side of `ProvidePair`
#[derive(Debug)]
pub struct IdxRight<I>(std::marker::PhantomData<I>);

impl<L, R> From<(L, R)> for ProvidePair<L, R> {
    fn from(pair: (L, R)) -> Self {
        let (left, right) = pair;
        Self { left, right }
    }
}

impl<'p, 'd, L, R, Output, NextProvideIdx> Provide<'p, 'd, Output, IdxLeft<NextProvideIdx>>
    for ProvidePair<L, R>
where
    Output: 'd,
    R: 'p,
    L: Provide<'p, 'd, Output, NextProvideIdx>,
{
    fn provide(&'p self) -> Output {
        self.left.provide()
    }
}

impl<'p, 'd, L, R, Output, NextProvideIdx> Provide<'p, 'd, Output, IdxRight<NextProvideIdx>>
    for ProvidePair<L, R>
where
    Output: 'd,
    L: 'p,
    R: Provide<'p, 'd, Output, NextProvideIdx>,
{
    fn provide(&'p self) -> Output {
        self.right.provide()
    }
}
