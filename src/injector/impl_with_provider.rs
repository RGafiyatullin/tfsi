use crate::components::ComponentForward;
use crate::components::ComponentFromProvider;
use crate::components::ComponentPair;
use crate::providers::ProvideForward;

use super::Injector;

impl<Components> Injector<Components> {
    /// Constructs an `Injector` by adding to `self` the given a `Component` over the given `provider: P`.
    pub fn with_provider<P>(
        self,
        provider: P,
    ) -> Injector<ComponentPair<ComponentFromProvider<P>, Components>> {
        self.with_component(ComponentFromProvider::from(provider))
    }

    /// Create an Injector that references `self` and another `provider: P`.
    pub fn ref_with_provider<'p, P>(
        &'p self,
        provider: &'p P,
    ) -> Injector<
        ComponentPair<
            ComponentForward<'p, Components>,
            ComponentFromProvider<ProvideForward<'p, P>>,
        >,
    > {
        let forward_left = ComponentForward::from(&self.components);
        let forward_right = ProvideForward::from(provider);
        let component_right = ComponentFromProvider::from(forward_right);
        let component_pair = ComponentPair::from((forward_left, component_right));
        Injector {
            components: component_pair,
        }
    }
}
