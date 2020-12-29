//!

/// A `Require` denotes the types of the required values.
///
/// A `Component` uses a `Require` to specify the types of the values necessary to instanticate its `Output` value.
pub trait Require<'p, 'd>: Sized {
    /// The type of a `Provide` that is expected when this requirement is fit.
    type Provider: 'p;

    /// Construct a `Self::Provider` from a given `P` if the latter fits this requirement.
    fn require<P, I>(p: &'p P) -> Self::Provider
    where
        P: FitRequirement<'p, 'd, Self, I>,
    {
        p.fit_requirement()
    }
}

/// Defines a way to fit some `R: Require`.
pub trait FitRequirement<'p, 'd, R, I>
where
    R: Require<'p, 'd>,
{
    ///
    fn fit_requirement(&'p self) -> <R as Require<'p, 'd>>::Provider;
}
