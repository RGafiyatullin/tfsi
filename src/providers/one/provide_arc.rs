use std::sync::Arc;

use crate::provide::Provide;

/// A `Provide` over an `Arc<Output>`.
///
/// If the `Value` is a `Clone`, the `ProvideArc<Value>` can provide `Value` (by cloning it) and `&Value`.
///
/// If the `Value` is not a `Clone`, the `ProvideArc<Value>` can only provide `&Value`.
#[derive(Debug)]
pub struct ProvideArc<Output>(Arc<Output>);

/// An Index to access `ProvideArc`.
#[derive(Debug)]
pub struct IdxProvideArc;

impl<T> Clone for ProvideArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.to_owned())
    }
}

impl<T> From<Arc<T>> for ProvideArc<T> {
    fn from(v: Arc<T>) -> Self {
        Self(v)
    }
}

impl<'p, 'd, Output> Provide<'p, 'd, Output, IdxProvideArc> for ProvideArc<Output>
where
    'd: 'p,
    Output: 'd,
    Output: Clone,
{
    fn provide(&'p self) -> Output {
        self.0.as_ref().to_owned()
    }
}

impl<'p, T> Provide<'p, 'p, &'p T, IdxProvideArc> for ProvideArc<T>
where
    T: 'p,
{
    fn provide(&'p self) -> &'p T {
        self.0.as_ref()
    }
}
