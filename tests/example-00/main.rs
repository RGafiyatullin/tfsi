#![recursion_limit = "16"]

#[macro_use]
extern crate tfsi;

use ::tfsi::prelude::*;
use providers::ProvideVal;

mod component_one;
use component_one::ComponentOne;

#[test]
fn main() {
    let injector = Injector::create().with_component(ComponentOne::default());

    let _c: component_one::C = injector
        .ref_with_provider(&ProvideVal::from("hello-1"))
        .provide();
}
