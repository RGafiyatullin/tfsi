use crate::components::ComponentForward;
use crate::components::ComponentPair;

use super::Injector;

impl<Components> Injector<Components> {
    /// Constructs an `Injector` by adding to `self` the given `component: C`.
    pub fn with_component<C>(self, component: C) -> Injector<ComponentPair<C, Components>> {
        Injector {
            components: ComponentPair::from((component, self.components)),
        }
    }

    /// Create an Injector that references `self` and another `component: C`.
    pub fn ref_with_component<'p, C>(
        &'p self,
        component: &'p C,
    ) -> Injector<ComponentPair<ComponentForward<'p, Components>, ComponentForward<'p, C>>> {
        let forward_left = ComponentForward::from(&self.components);
        let forward_right = ComponentForward::from(component);
        let component_pair = ComponentPair::from((forward_left, forward_right));
        Injector {
            components: component_pair,
        }
    }
}
