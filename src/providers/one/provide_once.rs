use std::cell::RefCell;

use crate::provide::Provide;

/// A `Provide` that is able to provide its value only once.
///
/// Contains a `RefCell` inside, therefore its `!Sync`.
#[derive(Debug)]
pub struct ProvideOnce<Value>(RefCell<Option<Value>>);

impl<Value> From<Value> for ProvideOnce<Value> {
    fn from(value: Value) -> Self {
        let opt = Some(value);
        let cell = RefCell::new(opt);
        Self(cell)
    }
}

/// An Index to access `ProvideOnce`.
#[derive(Debug)]
pub struct IdxProvideOnce;

impl<'p, 'd, Value> Provide<'p, 'd, Value, IdxProvideOnce> for ProvideOnce<Value>
where
    'd: 'p,
    Value: 'd,
{
    fn provide(&'p self) -> Value {
        self.0.replace(None).expect(
            "The value has already been taken. ProvideOnce can only produce its value once.",
        )
    }
}
