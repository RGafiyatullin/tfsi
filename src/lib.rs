#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![recursion_limit = "16"]
//! This is a compile-time dependency injection library.
//!
//!

pub mod component;
pub mod construct;
pub mod provide;
pub mod require;

pub mod injector;

pub mod components;
pub mod providers;
pub mod requirements;

#[macro_use]
mod macros;

pub mod prelude {
    //! A "prelude" for this crate's users.

    pub use crate::injector::Injector;

    pub use crate::component::Component;
    pub use crate::construct::Construct;
    pub use crate::provide::Provide;
    pub use crate::require::FitRequirement;
    pub use crate::require::Require;

    pub use crate::components;
    pub use crate::providers;
    pub use crate::requirements;
}
