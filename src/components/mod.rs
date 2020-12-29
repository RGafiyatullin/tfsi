//! A collection of standard `Component`s.

mod component_forward;
pub use component_forward::ComponentForward;
pub use component_forward::IdxComponentForward;

mod component_from_provider;
pub use component_from_provider::ComponentFromProvider;

mod component_pair;
pub use component_pair::ComponentPair;

#[cfg(test)]
mod component_basic_tests;
