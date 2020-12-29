
pub trait ProvideVal<T, I> {
    fn provide_val(&self) -> T;
}

pub trait ProvideRef<T, I> {
    fn provide_ref(&self) -> &T;
}
