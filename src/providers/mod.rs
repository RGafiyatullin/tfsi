//! A collection of standard `Provide`s.

mod provide_nil;
pub use provide_nil::ProvideNil;

mod one;
pub use one::IdxForward;
pub use one::IdxProvideArc;
pub use one::IdxProvideBoxed;
pub use one::IdxProvideOnce;
pub use one::IdxProvideVal;
pub use one::ProvideArc;
pub use one::ProvideBoxed;
pub use one::ProvideForward;
pub use one::ProvideOnce;
pub use one::ProvideVal;

mod pair;
pub use pair::IdxLeft;
pub use pair::IdxRight;
pub use pair::ProvidePair;
