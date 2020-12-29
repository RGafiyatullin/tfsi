use ::tfsi::prelude::*;
use requirements::RequireNil;

#[derive(Debug)]
pub struct A;
#[derive(Debug)]
pub struct B(A);

#[derive(Debug)]
pub struct C(B, String);

#[derive(Debug, Default)]
pub struct ComponentOne {}

impl<'p, 'd> Component<'p, 'd, A, Self> for ComponentOne
where
    'd: 'p,
{
    type Requirement = RequireNil;

    fn component_provide(&'p self, _deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> A {
        A
    }
}

impl<'p, 'd> Component<'p, 'd, B, Self> for ComponentOne
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, A];

    fn component_provide(&'p self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> B {
        B(deps.provide())
    }
}

impl<'p, 'd> Component<'p, 'd, C, Self> for ComponentOne
where
    'd: 'p,
{
    type Requirement = Require!['p, 'd, B, &'d str];

    fn component_provide(&'p self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> C {
        let s: &str = deps.provide();
        C(deps.provide(), s.to_owned())
    }
}
