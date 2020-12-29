use crate::providers::ProvidePair;
use crate::require::FitRequirement;
use crate::require::Require;

/// A type of `Require` that combines two other `Require`-types.
#[derive(Debug)]
pub struct RequirePair<Left, Right> {
    left: Left,
    right: Right,
}

#[derive(Debug)]
pub struct IdxRequirePair<LeftIdx, RightIdx>(std::marker::PhantomData<(LeftIdx, RightIdx)>);

impl<'p, 'd, Left, Right> Require<'p, 'd> for RequirePair<Left, Right>
where
    Left: Require<'p, 'd>,
    Right: Require<'p, 'd>,
{
    type Provider = ProvidePair<Left::Provider, Right::Provider>;
}

impl<'p, 'd, P, Left, Right, LeftIdx, RightIdx>
    FitRequirement<'p, 'd, RequirePair<Left, Right>, IdxRequirePair<LeftIdx, RightIdx>> for P
where
    Left: Require<'p, 'd>,
    Right: Require<'p, 'd>,
    P: FitRequirement<'p, 'd, Left, LeftIdx>,
    P: FitRequirement<'p, 'd, Right, RightIdx>,
    'd: 'p,
{
    fn fit_requirement(&'p self) -> <RequirePair<Left, Right> as Require<'p, 'd>>::Provider {
        let left = <Self as FitRequirement<'p, 'd, Left, LeftIdx>>::fit_requirement(self);
        let right = <Self as FitRequirement<'p, 'd, Right, RightIdx>>::fit_requirement(self);
        ProvidePair::from((left, right))
    }
}
