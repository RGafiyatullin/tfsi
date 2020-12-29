use std::sync::Arc;

use crate::provide::Provide;
use crate::providers::*;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;

    let a = ProvideVal::from(A);
    let b = ProvideVal::from(B);
    let ab = ProvidePair::from((a.to_owned(), b.to_owned()));
    let ba = ProvidePair::from((b.to_owned(), a.to_owned()));

    ref_via_ab::<A, B, _, _, _>(&ab);
    ref_via_ab::<A, B, _, _, _>(&ba);
    val_via_ab::<A, B, _, _, _>(&ab);
    val_via_ab::<A, B, _, _, _>(&ba);
}

#[test]
fn t02() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;

    let a = ProvideVal::from(A);
    let b = ProvideVal::from(B);
    let ab = ProvidePair::from((ProvideForward::from(&a), ProvideForward::from(&b)));
    let ba = ProvidePair::from((ProvideForward::from(&b), ProvideForward::from(&a)));

    ref_via_ab::<A, B, _, _, _>(&ab);
    ref_via_ab::<A, B, _, _, _>(&ba);
    val_via_ab::<A, B, _, _, _>(&ab);
    val_via_ab::<A, B, _, _, _>(&ba);
}

#[test]
fn t03() {
    struct A;
    struct B;
    struct C;

    let a = ProvideVal::from(&A);
    let b = ProvideVal::from(B);
    let c = ProvideArc::from(Arc::new(C));

    let abc_l = ProvidePair::from((
        ProvideForward::from(&a),
        ProvidePair::from((ProvideForward::from(&b), ProvideForward::from(&c))),
    ));
    let abc_r = ProvidePair::from((
        ProvidePair::from((ProvideForward::from(&a), ProvideForward::from(&b))),
        ProvideForward::from(&c),
    ));

    ref_via_abc::<A, B, C, _, _, _, _>(&abc_l);
    ref_via_abc::<A, B, C, _, _, _, _>(&abc_r);
}

#[test]
fn t04() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;
    #[derive(Clone)]
    struct C;

    let a = ProvideVal::from(A);
    let b = ProvideVal::from(B);
    let c = ProvideArc::from(Arc::new(C));

    let abc_l = ProvidePair::from((
        ProvideForward::from(&a),
        ProvidePair::from((ProvideForward::from(&b), ProvideForward::from(&c))),
    ));
    let abc_r = ProvidePair::from((
        ProvidePair::from((ProvideForward::from(&a), ProvideForward::from(&b))),
        ProvideForward::from(&c),
    ));

    ref_via_abc::<A, B, C, _, _, _, _>(&abc_l);
    ref_via_abc::<A, B, C, _, _, _, _>(&abc_r);
    val_via_abc::<A, B, C, _, _, _, _>(&abc_l);
    val_via_abc::<A, B, C, _, _, _, _>(&abc_r);
}

#[test]
fn t05() {
    struct A;
    struct B;

    fn inner<'data, T1: 'static, T2: 'static>(
        data_1: &'data T1,
        data_2: &'data T2,
    ) -> (&'data T1, &'data T2) {
        let p1 = ProvideVal::from(data_1);
        let p2 = ProvideVal::from(data_2);
        let p = ProvidePair::from((ProvideForward::from(&p1), ProvideForward::from(&p2)));

        (p.provide(), p.provide())
    }

    let (_ref_a, _ref_b): (&A, &B) = inner(&A, &B);
}

fn ref_via_ab<'p, 'd, A, B, AI, BI, P>(p: &'p P)
where
    // 'd: 'p,
    P: Provide<'p, 'd, &'d A, AI>,
    P: Provide<'p, 'd, &'d B, BI>,
    A: 'd,
    B: 'd,
{
    let _ref_a: &A = p.provide();
    let _ref_b: &B = p.provide();
}

fn val_via_ab<'p, 'd, A, B, AI, BI, P>(p: &'p P)
where
    // 'd: 'p,
    P: Provide<'p, 'd, A, AI>,
    P: Provide<'p, 'd, B, BI>,
    A: 'd,
    B: 'd,
{
    let _val_a: A = p.provide();
    let _val_b: B = p.provide();
}

fn ref_via_abc<'p, 'd, A, B, C, AI, BI, CI, P>(p: &'p P)
where
    // 'd: 'p,
    P: Provide<'p, 'd, &'d A, AI>,
    P: Provide<'p, 'd, &'d B, BI>,
    P: Provide<'p, 'd, &'d C, CI>,
    A: 'd,
    B: 'd,
    C: 'd,
{
    let _ref_a: &A = p.provide();
    let _ref_b: &B = p.provide();
    let _ref_b: &C = p.provide();
}

fn val_via_abc<'p, 'd, A, B, C, AI, BI, CI, P>(p: &'p P)
where
    // 'd: 'p,
    P: Provide<'p, 'd, A, AI>,
    P: Provide<'p, 'd, B, BI>,
    P: Provide<'p, 'd, C, CI>,
    A: 'd,
    B: 'd,
    C: 'd,
{
    let _val_a: A = p.provide();
    let _val_b: B = p.provide();
    let _val_b: C = p.provide();
}
