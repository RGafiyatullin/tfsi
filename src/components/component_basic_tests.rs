use crate::prelude::*;
#[test]
fn t01() {
    use requirements::*;

    #[derive(Clone)]
    struct A;

    #[derive(Clone)]
    struct B;

    #[derive(Clone)]
    struct Ab(A, B);

    struct TheComponent;

    impl<'p, 'd> Component<'p, 'd, A, ()> for TheComponent {
        type Requirement = RequireNil;

        fn component_provide(&self, _deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> A {
            A
        }
    }

    impl<'p, 'd> Component<'p, 'd, B, ()> for TheComponent {
        type Requirement = RequireNil;

        fn component_provide(&self, _deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> B {
            B
        }
    }

    impl<'p, 'd> Component<'p, 'd, Ab, ()> for TheComponent
    where
        'd: 'p,
    {
        type Requirement = RequirePair<RequireOne<'p, 'd, A>, RequireOne<'p, 'd, B>>;

        fn component_provide(&self, deps: &<Self::Requirement as Require<'p, 'd>>::Provider) -> Ab {
            let a = deps.provide();
            let b = deps.provide();
            Ab(a, b)
        }
    }

    let _val_a: A = TheComponent.provide();
    let _val_b: B = TheComponent.provide();
    let _val_ab: Ab = TheComponent.provide();
}
