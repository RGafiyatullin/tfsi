use crate::prelude::Provide;

/// A `Provide` over some other `Provide` that erases the `ProvideIdx` data type.
pub struct ProvideBoxed<'p, 'd, Output>(Box<dyn ProvideBridged<'d, Output> + 'p>);

impl<'p, 'd, T> std::fmt::Debug for ProvideBoxed<'p, 'd, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProvideOneValBoxed")
    }
}

/// An Index to access the `ProvideBoxed`.
#[derive(Debug)]
pub struct IdxProvideBoxed;

impl<'p, 'd, Output> ProvideBoxed<'p, 'd, Output> {
    /// Create a `ProvideBoxed<Output>` from a `Provide<Output, ...>
    pub fn create<P, I>(p: &'p P) -> Self
    where
        P: Provide<'p, 'd, Output, I>,
        I: 'p,
        Output: 'd,
    {
        let p = ProvideBridge::<'p, P, I> {
            provider: p,
            _pd: Default::default(),
        };
        let p: Box<dyn ProvideBridged<'d, Output> + 'p> = Box::new(p);
        Self(p)
    }
}

impl<'p, 'd, Output> Provide<'p, 'd, Output, IdxProvideBoxed> for ProvideBoxed<'p, 'd, Output>
where
    Self: 'p,
    Output: 'd,
{
    fn provide(&'p self) -> Output {
        self.0.provide_bridged()
    }
}

struct ProvideBridge<'p, P, I> {
    provider: &'p P,
    _pd: std::marker::PhantomData<I>,
}

trait ProvideBridged<'d, T>
where
    T: 'd,
{
    fn provide_bridged(&self) -> T;
}

impl<'p, 'd, P, T, I> ProvideBridged<'d, T> for ProvideBridge<'p, P, I>
where
    P: Provide<'p, 'd, T, I>,
    I: 'p,
    T: 'd,
{
    fn provide_bridged(&self) -> T {
        self.provider.provide()
    }
}
