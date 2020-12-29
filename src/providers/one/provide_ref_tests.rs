use crate::prelude::*;
use crate::providers::ProvideVal;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;
    let p = ProvideVal::from(&A);
    // let _val_a1: A = p.provide();
    // let _val_a2: A = p.provide();
    let _ref_a1: &A = p.provide();
    let _ref_a2: &A = p.provide();
}

#[test]
fn t02() {
    struct A;
    let p = ProvideVal::from(&A);
    let _ref_a1: &A = p.provide();
    let _ref_a2: &A = p.provide();
}

#[test]
fn t03() {
    struct A;
    fn inner<'data, T>(data: &'data T) -> &'data T {
        let p = ProvideVal::from(data);
        p.provide()
    }

    let _ref_a = inner(&A);
}
