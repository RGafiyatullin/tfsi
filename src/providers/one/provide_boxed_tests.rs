use crate::prelude::*;
use providers::*;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;

    let p = ProvideVal::from(A);
    let p_boxed = ProvideBoxed::<A>::create(&p);

    let _val_a: A = p_boxed.provide();
}

#[test]
fn t02() {
    #[derive(Clone)]
    struct A;

    #[derive(Clone)]
    struct B;

    let p_a = ProvideVal::from(A);
    let p_b = ProvideVal::from(B);
    let p_ab = ProvidePair::from((ProvideForward::from(&p_a), ProvideForward::from(&p_b)));

    let p_boxed_a = ProvideBoxed::<A>::create(&p_ab);
    let p_boxed_b = ProvideBoxed::<B>::create(&p_ab);

    let _val_a: A = p_boxed_a.provide();
    let _val_b: B = p_boxed_b.provide();
}
