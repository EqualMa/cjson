//! Currently this test is not relied on.

struct Matched<const N: u8>;

macro_rules! test_match_ref_lt {
    (($runtime_expr:expr) as &'_ $Ty:ty) => {
        1
    };
    (($runtime_expr:expr) as & $lt:lifetime $Ty:ty) => {
        2
    };
    (($runtime_expr:expr) as & $Ty:ty) => {
        3
    };
    (($runtime_expr:expr) as $Ty:ty) => {
        4
    };
    (($runtime_expr:expr)) => {
        5
    };
}

const _: () = {
    let Matched::<3> = Matched::<
        {
            test_match_ref_lt! { (_) as &_ }
        },
    >;
    let Matched::<1> = Matched::<
        {
            test_match_ref_lt! { (_) as &'_ _ }
        },
    >;
    let Matched::<2> = Matched::<
        {
            test_match_ref_lt! { (_) as &'a _ }
        },
    >;

    macro_rules! rematch_ty {
        ($runtime_expr:tt as $ty:ty) => {
            test_match_ref_lt! { $runtime_expr as $ty }
        };
    }

    let Matched::<4> = Matched::<
        {
            rematch_ty! { (_) as &_ }
        },
    >;

    let Matched::<5> = Matched::<
        {
            test_match_ref_lt! { (_) }
        },
    >;
};

#[test]
fn compile_only() {}
