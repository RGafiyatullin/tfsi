#[test]
fn t02() {
    trait I<'p, 'd, T: 'd> {
        fn call(&'p self) -> T;
    }

    fn invoke_and_drop<'d, P, T>(p: P) -> T
    where
        P: for<'p_local> I<'p_local, 'd, T>,
        T: 'd,
    {
        p.call()
    }

    #[derive(Clone)]
    struct A;
    impl<'p> I<'p, '_, A> for A {
        fn call(&'p self) -> A {
            self.clone()
        }
    }
    struct B<'a>(&'a A);
    impl<'p, 'd> I<'p, 'd, &'d A> for B<'d>
    where
        'd: 'p,
    {
        fn call(&'p self) -> &'d A {
            &self.0
        }
    }

    let _ref_a: A = A.call();
    let _ref_a: &A = B(&A).call();

    let _ref_a: A = invoke_and_drop(A);
    let _ref_a: &A = invoke_and_drop(B(&A));
}

/*
error: implementation of `I` is not general enough
  --> tests/playground.rs:33:22
   |
3  | /     trait I<'p, 'd, T: 'd>: 'p {
4  | |         fn call(&'p self) -> T;
5  | |     }
   | |_____- trait `I` defined here
...
33 |       let _ref_a: &A = invoke_and_drop(A);
   |                        ^^^^^^^^^^^^^^^ implementation of `I` is not general enough
   |
   = note: `A` must implement `I<'0, '1, &A>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `A` actually implements `I<'2, '2, &'2 A>`, for some specific lifetime `'2`

error: implementation of `I` is not general enough
  --> tests/playground.rs:34:22
   |
3  | /     trait I<'p, 'd, T: 'd>: 'p {
4  | |         fn call(&'p self) -> T;
5  | |     }
   | |_____- trait `I` defined here
...
34 |       let _ref_a: &A = invoke_and_drop(B(&A));
   |                        ^^^^^^^^^^^^^^^ implementation of `I` is not general enough
   |
   = note: `I<'0, '1, &A>` would have to be implemented for the type `B<'_>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `I<'_, '2, &'2 A>` is actually implemented for the type `B<'2>`, for some specific lifetime `'2`
*/
