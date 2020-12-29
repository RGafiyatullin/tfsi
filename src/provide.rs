//!

/// A `Provide` — a strategy to instantiate a value without any prerequisites.
pub trait Provide<'provider, 'data, Output, ProviderIdx>: 'provider
where
    Output: 'data,
{
    /// Yield the provided `Output`.
    fn provide(&'provider self) -> Output;
}
