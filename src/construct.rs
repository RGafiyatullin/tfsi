//!

use crate::require::Require;

/// A `Construct` is a strategy to instantiate `Self` using the provider satisfying the `Self::Requirement`.
///
/// A type may have several `Construct` strategies: they need to have different `Idx`.
pub trait Construct<'p, 'd, Idx> {
    /// The `Require` necessary to instantiate `Self` via this `Construct`
    type Requirement: Require<'p, 'd>;

    /// Construct a `Self`: `args` — a provider satisfying the `Self::Requirement`.
    fn construct(args: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Self;
}
