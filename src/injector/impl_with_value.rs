use crate::components::ComponentFromProvider;
use crate::components::ComponentPair;
use crate::providers::ProvideVal;

use super::Injector;

impl<Components> Injector<Components> {
    /// Constructs an `Injector` by adding to `self` a component that provides the given `value: V`.
    pub fn with_value<V>(
        self,
        value: V,
    ) -> Injector<ComponentPair<ComponentFromProvider<ProvideVal<V>>, Components>> {
        self.with_component(ComponentFromProvider::from(ProvideVal::from(value)))
    }
}
