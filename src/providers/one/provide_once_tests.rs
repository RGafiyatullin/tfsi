use crate::prelude::*;
use providers::*;

#[test]
fn t01() {
    struct A;

    let p = ProvideOnce::from(A);

    let _val_a: A = p.provide();
}

#[test]
#[should_panic]
fn t02() {
    struct A;

    let p = ProvideOnce::from(A);

    let _val_a: A = p.provide();
    let _val_a: A = p.provide();
}
