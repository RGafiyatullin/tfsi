use crate::provide::Provide;

use super::Injector;

/// An Index to access the `Injector`
#[derive(Debug)]
pub struct IdxInjector<I>(std::marker::PhantomData<I>);

impl<'p, 'd, C, T, I> Provide<'p, 'd, T, IdxInjector<I>> for Injector<C>
where
    C: Provide<'p, 'd, T, I>,
    T: 'd,
{
    fn provide(&'p self) -> T {
        self.components.provide()
    }
}
