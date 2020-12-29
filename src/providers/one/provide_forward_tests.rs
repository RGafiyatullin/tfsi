use one::ProvideForward;

use crate::prelude::*;
use crate::providers::*;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;

    let p_a = ProvideVal::from(A);
    let _val_a: A = p_a.provide();
    let p_a_fw = ProvideForward::from(&p_a);
    let _val_a: A = p_a_fw.provide();
}

#[test]
fn t02() {
    struct A;

    let p_a = ProvideVal::from(&A);
    let _ref_a: &A = p_a.provide();
    let p_a_fw = ProvideForward::from(&p_a);
    let _ref_a: &A = p_a_fw.provide();
}

#[test]
fn t03() {
    #[derive(Clone)]
    struct A;

    #[derive(Clone)]
    struct B;

    let p_a = ProvideVal::from(A);
    let p_b = ProvideVal::from(B);
    let p_ab = ProvidePair::from((p_a, p_b));
    let _val_a: A = p_ab.provide();
    let _val_b: B = p_ab.provide();
    let p_ab_fw = ProvideForward::from(&p_ab);
    let _val_a: A = p_ab_fw.provide();
    let _val_a: B = p_ab_fw.provide();
}
