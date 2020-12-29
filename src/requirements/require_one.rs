use crate::provide::Provide;
use crate::providers::ProvideBoxed;
use crate::require::FitRequirement;
use crate::require::Require;

/// A type of `Require` that requires a single type.
#[derive(Debug)]
pub struct RequireOne<'p, 'd, T>(std::marker::PhantomData<(&'p (), &'d (), T)>);

impl<'p, 'd, T> Require<'p, 'd> for RequireOne<'p, 'd, T>
where
    T: 'd,
    'd: 'p,
{
    type Provider = ProvideBoxed<'p, 'd, T>;
}

impl<'p, 'd, P, T, I> FitRequirement<'p, 'd, RequireOne<'p, 'd, T>, I> for P
where
    P: Provide<'p, 'd, T, I>,
    T: 'd,
    I: 'p,
    'd: 'p,
{
    fn fit_requirement(&'p self) -> <RequireOne<'p, 'd, T> as Require<'p, 'd>>::Provider {
        ProvideBoxed::<'p, 'd, T>::create(self)
    }
}
