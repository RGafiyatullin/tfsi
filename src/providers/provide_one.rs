
use crate::provide::ProvideVal;
use crate::provide::ProvideRef;

#[derive(Debug)]
pub struct ProvideOne<T>(T);

impl<T> From<T> for ProvideOne<T> 
{
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> ProvideVal<T, Self> for ProvideOne<T> 
    where T: Clone
{
    fn provide_val(&self) -> T {
        self.0.to_owned()
    }
}

impl<T> ProvideRef<T, Self> for ProvideOne<T> {
    fn provide_ref(&self) -> &T {
        &self.0
    }
}

