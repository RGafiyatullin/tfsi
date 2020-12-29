use crate::components::ComponentFromProvider;
use crate::providers::ProvideNil;

use super::Injector;

impl Injector<ComponentFromProvider<ProvideNil>> {
    /// Constructs an empty `Injector`
    pub fn create() -> Self {
        Self {
            components: ComponentFromProvider::from(ProvideNil),
        }
    }
}
