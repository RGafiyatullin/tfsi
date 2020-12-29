//!

mod impl_construct;
mod impl_empty;
mod impl_trait_provide;
mod impl_with_arc;
mod impl_with_component;
mod impl_with_provider;
mod impl_with_value;

pub use impl_trait_provide::IdxInjector;

/// An `Injector`: a container for components.
#[derive(Debug, Clone)]
pub struct Injector<Components> {
    components: Components,
}

#[cfg(test)]
mod injector_tests;
