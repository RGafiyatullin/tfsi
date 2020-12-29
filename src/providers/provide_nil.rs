use crate::provide::Provide;

/// A `Provide` that can only provide a unit (`()`).
///
/// Used as a `Require::Provider` for `RequireNil`.
#[derive(Debug, Clone, Copy)]
pub struct ProvideNil;

struct IdxProvideNil;

impl<'p> Provide<'p, 'static, (), IdxProvideNil> for ProvideNil {
    fn provide(&'p self) -> () {
        ()
    }
}
