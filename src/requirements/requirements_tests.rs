use crate::prelude::*;
use crate::require::Require;
use providers::*;
use requirements::*;

#[test]
fn t_02() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;
    #[derive(Clone)]
    struct C;
    #[derive(Clone)]
    struct D;

    let p_a = ProvideVal::from(A);
    let p_b = ProvideVal::from(B);
    let p_c = ProvideVal::from(C);
    let p_d = ProvideVal::from(D);

    let p_ab = ProvidePair::from((ProvideForward::from(&p_a), ProvideForward::from(&p_b)));
    let p_cd = ProvidePair::from((ProvideForward::from(&p_c), ProvideForward::from(&p_d)));

    let p_abcd = ProvidePair::from((ProvideForward::from(&p_ab), ProvideForward::from(&p_cd)));

    let _p_nil = RequireNil::require(&p_abcd);

    let p_a = RequireOne::<A>::require(&p_abcd);
    let _val_a: A = p_a.provide();

    let p_b = RequireOne::<B>::require(&p_abcd);
    let _val_b: B = p_b.provide();

    let p_c = RequireOne::<C>::require(&p_abcd);
    let _val_c: C = p_c.provide();

    let p_d = RequireOne::<D>::require(&p_abcd);
    let _val_d: D = p_d.provide();

    let p_ad = RequirePair::<RequireOne<A>, RequireOne<D>>::require(&p_abcd);
    let _val_a: A = p_ad.provide();
    let _val_d: D = p_ad.provide();

    let p_bc = RequirePair::<RequireOne<B>, RequireOne<C>>::require(&p_abcd);
    let _val_b: B = p_bc.provide();
    let _val_c: C = p_bc.provide();

    let p_abcd = RequirePair::<
        RequirePair<RequireOne<A>, RequireOne<D>>,
        RequirePair<RequireOne<B>, RequireOne<C>>,
    >::require(&p_abcd);

    let _val_a: A = p_abcd.provide();
    let _val_b: B = p_abcd.provide();
    let _val_c: C = p_abcd.provide();
    let _val_d: D = p_abcd.provide();
}
