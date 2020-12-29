//! A collection of standard `Require`s.

mod require_one;
pub use require_one::RequireOne;

mod require_pair;
pub use require_pair::RequirePair;

mod require_nil;
pub use require_nil::RequireNil;

#[cfg(test)]
mod requirements_tests;
