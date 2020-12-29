use crate::providers::ProvideNil;
use crate::require::FitRequirement;
use crate::require::Require;

/// A type of `Require` that requires nothing.
///
/// Technically it requires a unit (`()`): this requirement can be fulfilled from any provider.
#[derive(Debug)]
pub struct RequireNil;

#[derive(Debug)]
pub struct IdxRequireNil;

impl Require<'_, '_> for RequireNil {
    type Provider = ProvideNil;
}

impl<Any> FitRequirement<'_, '_, RequireNil, IdxRequireNil> for Any {
    fn fit_requirement(&'_ self) -> <RequireNil as Require>::Provider {
        ProvideNil
    }
}
