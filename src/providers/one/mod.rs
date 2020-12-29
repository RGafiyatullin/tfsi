mod provide_val;
pub use provide_val::IdxProvideVal;
pub use provide_val::ProvideVal;

mod provide_arc;
pub use provide_arc::IdxProvideArc;
pub use provide_arc::ProvideArc;

mod provide_forward;
pub use provide_forward::IdxForward;
pub use provide_forward::ProvideForward;

mod provide_boxed;
pub use provide_boxed::IdxProvideBoxed;
pub use provide_boxed::ProvideBoxed;

mod provide_once;
pub use provide_once::IdxProvideOnce;
pub use provide_once::ProvideOnce;

#[cfg(test)]
mod provide_arc_tests;
#[cfg(test)]
mod provide_boxed_tests;
#[cfg(test)]
mod provide_forward_tests;
#[cfg(test)]
mod provide_once_tests;
#[cfg(test)]
mod provide_ref_tests;
#[cfg(test)]
mod provide_val_tests;
