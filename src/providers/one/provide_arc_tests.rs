use std::sync::Arc;

use crate::prelude::*;
use crate::providers::ProvideArc;

#[test]
fn t01() {
    #[derive(Clone)]
    struct A;

    let p = ProvideArc::from(Arc::new(A));
    let _val_a1: A = p.provide();
    let _val_a2: A = p.provide();
    let _ref_a1: &A = p.provide();
    let _ref_a2: &A = p.provide();
    let _ref_a3: &A = p.provide();
}

#[test]
fn t02() {
    struct A;

    let p = ProvideArc::from(Arc::new(A));
    let _ref_a1: &A = p.provide();
    let _ref_a2: &A = p.provide();
}

#[test]
fn t03() {
    struct A;

    let p1 = ProvideArc::from(Arc::new(A));
    let p2 = p1.clone();

    let _ref_a1: &A = p1.provide();
    let _ref_a2: &A = p2.provide();
}
