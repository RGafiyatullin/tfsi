// use crate::provide::ProvideRef;
use crate::provide::Provide;

/// A `Provide` over some other `Provide`.
///
/// Can be used to share `Provide`s.
#[derive(Debug)]
pub struct ProvideForward<'p, SharedProvider>(&'p SharedProvider);

impl<'p, P> From<&'p P> for ProvideForward<'p, P> {
    fn from(p: &'p P) -> Self {
        Self(p)
    }
}

/// An Index to access `ProvideForward`
#[derive(Debug)]
pub struct IdxForward<I>(std::marker::PhantomData<I>);

impl<'p, P> Clone for ProvideForward<'p, P> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'p, 'd, P, Output, I> Provide<'p, 'd, Output, IdxForward<I>> for ProvideForward<'p, P>
where
    Output: 'd,
    P: Provide<'p, 'd, Output, I>,
{
    fn provide(&'p self) -> Output {
        self.0.provide()
    }
}
