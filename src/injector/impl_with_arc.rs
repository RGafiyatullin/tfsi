use std::sync::Arc;

use crate::components::ComponentFromProvider;
use crate::components::ComponentPair;
use crate::providers::ProvideArc;

use super::Injector;

impl<Components> Injector<Components> {
    /// Constructs an `Injector` by adding to `self` a component that provides the access to the value via the given `Arc<V>`.
    pub fn with_arc<V>(
        self,
        arced_value: Arc<V>,
    ) -> Injector<ComponentPair<ComponentFromProvider<ProvideArc<V>>, Components>> {
        self.with_component(ComponentFromProvider::from(ProvideArc::from(arced_value)))
    }
}
