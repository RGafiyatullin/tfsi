
use crate::prelude::*;
use crate::providers::ProvideOne;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;
    let p = ProvideOne::from(A);
    let _val_a1: A = p.provide_val();
    let _val_a2: A = p.provide_val();
    let _ref_a1: &A = p.provide_ref();
    let _ref_a2: &A = p.provide_ref();
}

#[test]
fn t02() {
    struct A;
    let p = ProvideOne::from(A);
    let _ref_a1: &A = p.provide_ref();
    let _ref_a2: &A = p.provide_ref();
}