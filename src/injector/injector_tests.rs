use crate::prelude::*;
use components::*;
use providers::*;
use requirements::*;

#[test]
fn t01() {
    #[derive(Debug)]
    struct A(&'static str);
    #[derive(Debug)]
    struct B(&'static str);
    #[derive(Debug)]
    struct ComponentAB(&'static str);

    let ab_component = ComponentAB("AB");

    #[derive(Debug, Clone)]
    struct C;
    let c_provider = ProvideVal::from(C);

    #[derive(Debug)]
    struct D(A, B, C);

    #[derive(Debug)]
    struct ComponentD;

    let d_component = ComponentD;

    {
        impl<'p, 'd> Component<'p, 'd, A, ()> for ComponentAB {
            type Requirement = RequireNil;

            fn component_provide(
                &'p self,
                _deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
            ) -> A {
                A(self.0)
            }
        }

        impl<'p, 'd> Component<'p, 'd, B, ()> for ComponentAB {
            type Requirement = RequireNil;

            fn component_provide(
                &'p self,
                _deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
            ) -> B {
                B(self.0)
            }
        }

        impl<'p, 'd> Component<'p, 'd, D, ()> for ComponentAB
        where
            'd: 'p,
        {
            type Requirement = RequirePair<
                RequireOne<'p, 'd, C>,
                RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>,
            >;

            fn component_provide(
                &'p self,
                deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
            ) -> D {
                let a = deps.provide();
                let b = deps.provide();
                let c = deps.provide();
                D(a, b, c)
            }
        }
    }

    let injector = Injector::create()
        .with_component(d_component)
        .with_component(ab_component)
        .with_provider(c_provider);

    let a: A = injector.provide();
    println!("A: {:?}", a);
    let b: B = injector.provide();
    println!("B: {:?}", b);
    let c: C = injector.provide();
    println!("C: {:?}", c);
    let d: D = injector.provide();
    println!("D: {:?}", d);
}

#[test]
fn t02() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;

    struct C(A, B);
    impl<'p, 'd> Construct<'p, 'd, Self> for C
    where
        'd: 'p,
    {
        type Requirement = RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>;

        fn construct(args: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Self {
            Self(args.provide(), args.provide())
        }
    }

    let p_a = ProvideVal::from(A);
    let p_b = ProvideVal::from(B);

    let injector = Injector::create().with_provider(p_a).with_provider(p_b);
    let _c: C = injector.construct();
}

#[test]
fn t03() {
    #[derive(Debug, Clone)]
    struct A;
    #[derive(Debug, Clone)]
    struct B;

    #[derive(Debug)]
    struct C(A, B);

    impl<'p, 'd> Construct<'p, 'd, C> for C
    where
        'd: 'p,
    {
        type Requirement = RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>;

        fn construct(args: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Self {
            Self(args.provide(), args.provide())
        }
    }

    let provide_a = ProvideVal::from(A);
    let provide_b = ProvideVal::from(B);
    let component_b = ComponentFromProvider::from(provide_b);
    let injector = Injector::create().with_provider(provide_a);

    let tmp_injector = injector.ref_with_component(&component_b);

    let _a: A = injector.provide();
    let _b: B = tmp_injector.provide();
    let _c: C = tmp_injector.construct();
}

#[test]
fn t04() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;

    struct C(A, B);
    impl<'p, 'd> Construct<'p, 'd, Self> for C
    where
        'd: 'p,
    {
        type Requirement = RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>;

        fn construct(args: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Self {
            Self(args.provide(), args.provide())
        }
    }

    let p_a = ProvideVal::from(A);
    let p_b_once = ProvideOnce::from(B);

    let injector = Injector::create().with_provider(p_a);
    let _c: C = injector.ref_with_provider(&p_b_once).construct();
}

#[test]
fn t05() {
    #[derive(Clone)]
    struct A;
    #[derive(Clone)]
    struct B;
    struct ComponentB;

    impl<'p, 'd> Component<'p, 'd, B, Self> for ComponentB {
        type Requirement = RequireNil;

        fn component_provide(
            &'p self,
            _deps: &<Self::Requirement as Require<'p, 'd>>::Provider,
        ) -> B {
            B
        }
    }

    struct C(A, B);
    impl<'p, 'd> Construct<'p, 'd, Self> for C
    where
        'd: 'p,
    {
        type Requirement = RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>;

        fn construct(args: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Self {
            Self(args.provide(), args.provide())
        }
    }

    let p_a = ProvideVal::from(A);

    let injector = Injector::create().with_provider(p_a);
    let _c: C = injector.ref_with_component(&ComponentB).construct();
}
