use crate::construct::Construct;
use crate::require::FitRequirement;
use crate::require::Require;

use super::Injector;

impl<Components> Injector<Components> {
    /// Instantiate an `Output` using this `Injector` via `Construct<ConstructIdx>`
    pub fn construct<'p, 'd, Output, ConstructIdx, ProvideIdx>(&'p self) -> Output
    where
        'd: 'p,
        Output: Construct<'p, 'd, ConstructIdx>,
        Components: 'p,
        Self: FitRequirement<
            'p,
            'd,
            <Output as Construct<'p, 'd, ConstructIdx>>::Requirement,
            ProvideIdx,
        >,
    {
        let args = <Output::Requirement as Require>::require(self);
        Output::construct(&args)
    }
}
