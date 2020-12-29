use crate::provide::Provide;

/// A `Provide` over the given `Value`.
///
/// If the `Value` is a `Clone`, the `ProvideVal<Value>` can provide `Value` (by cloning it) and `&Value`.
///
/// If the `Value` is not a `Clone`, the `ProvideVal<Value>` can only provide `&Value`.
///
/// Thus the `Provide<&Value>` can be used to provide a reference to some `Value` without consuming it.
#[derive(Debug, Clone)]
pub struct ProvideVal<Value>(Value);

/// An Index to access `ProvideVal`.
#[derive(Debug)]
pub struct IdxProvideVal;

impl<T> From<T> for ProvideVal<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<'p, 'd, Output> Provide<'p, 'd, Output, IdxProvideVal> for ProvideVal<Output>
where
    'd: 'p,
    Output: 'd,
    Output: Clone,
{
    fn provide(&'p self) -> Output {
        self.0.to_owned()
    }
}

impl<'p, Output> Provide<'p, 'p, &'p Output, IdxProvideVal> for ProvideVal<Output>
where
    Output: 'p,
{
    fn provide(&'p self) -> &'p Output {
        &self.0
    }
}
