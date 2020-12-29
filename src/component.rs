//!

use crate::provide::Provide;
use crate::require::FitRequirement;
use crate::require::Require;

/// A `Component` — a strategy to instantiate a value using the declared prerequisites.
///
/// The `Component`'s associated type `Self::Requirement` is used to construct a `Provide` for the dependencies necessary for instantication of `Output`.
/// The said `Provide` is passed to the `Component::component_provide(...)` method upon invocation of the `Component`.
pub trait Component<'p, 'd, Output, ComponentIdx> {
    ///
    type Requirement: Require<'p, 'd>;

    ///
    fn component_provide(
        &'p self,
        deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
    ) -> Output;
}

impl<'p, 'd, AComponent, Output, ComponentIdx, DependenciesIdx>
    Provide<'p, 'd, Output, (ComponentIdx, DependenciesIdx)> for AComponent
where
    AComponent: 'p,
    Output: 'd,
    AComponent: Component<'p, 'd, Output, ComponentIdx>,
    AComponent: FitRequirement<
        'p,
        'd,
        <AComponent as Component<'p, 'd, Output, ComponentIdx>>::Requirement,
        DependenciesIdx,
    >,
{
    fn provide(&'p self) -> Output {
        let deps = self.fit_requirement();
        self.component_provide(&deps)
    }
}
