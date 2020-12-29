/// An utility macro to construct a compound `Require`.
#[macro_export]
macro_rules! Require {
    [ $lt_p: lifetime, $lt_d: lifetime ] => { $crate::requirements::RequireNil, };
    [ $lt_p: lifetime, $lt_d: lifetime, $head: ty ] => { $crate::requirements::RequireOne<$lt_p, $lt_d, $head> };
    [ $lt_p: lifetime, $lt_d: lifetime, $head: ty, $($tail: ty), * ] => {
        $crate::requirements::RequirePair<
            $crate::requirements::RequireOne<$lt_p, $lt_d, $head>,
            $crate::Require![ $lt_p, $lt_d, $($tail), *]
        > };
    [ $lt_p: lifetime, $lt_d: lifetime, $head: ty, $($tail: ty), *, ] => {
        $crate::requirements::RequirePair<
            $crate::requirements::RequireOne<$lt_p, $lt_d, $head>,
            $crate::Require![ $lt_p, $lt_d, $($tail), *]
        > };
}
