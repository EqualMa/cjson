use core::{fmt::Write as _, mem::MaybeUninit};

use crate::utils::impl_many;

type ArrayString<const CAP: usize> = crate::r#const::array_string::ArrayString<u8, CAP>;

pub(super) fn int_to_string<const CAP: usize>(v: impl core::fmt::Display) -> ArrayString<CAP> {
    let mut ret = ArrayString::NEW;
    write!(ret, "{}", v).unwrap();

    ret
}

pub(super) fn format<const CAP: usize>(
    // MaybeUninit to make sure we create the empty ArrayString manually
    buf: &mut MaybeUninit<ArrayString<CAP>>,
    i: impl core::fmt::Display,
) -> &str {
    let buf = buf.write(ArrayString::NEW);
    write!(buf, "{}", i).unwrap();

    buf.as_str()
}

pub(super) trait Integer {
    const MAX_STR_LEN: usize;
}

impl_many!(
    impl<__> Integer for each_of![i8, i16, i32, i64, isize, i128] {
        const MAX_STR_LEN: usize = Self::MAX.ilog10() as usize + 2;
    }
);

impl_many!(
    impl<__> Integer for each_of![u8, u16, u32, u64, usize, u128] {
        const MAX_STR_LEN: usize = Self::MAX.ilog10() as usize + 1;
    }
);

#[cfg(test)]
#[cfg(feature = "itoa")]
mod tests {
    use super::{super::impl_itoa, Integer, int_to_string};

    const _: () = {
        const fn assert_same<T: Integer + itoa::Integer>() {
            assert!(<T as Integer>::MAX_STR_LEN == <T as itoa::Integer>::MAX_STR_LEN)
        }

        assert_same::<i8>();
        assert_same::<i16>();
        assert_same::<i32>();
        assert_same::<i64>();
        assert_same::<isize>();
        assert_same::<i128>();
        assert_same::<u8>();
        assert_same::<u16>();
        assert_same::<u32>();
        assert_same::<u64>();
        assert_same::<usize>();
        assert_same::<u128>();
    };

    macro_rules! assert_same_for_expr {
        ([$Ty:ty] $v:expr) => {{
            const CAP1: usize = <$Ty as Integer>::MAX_STR_LEN;
            const CAP2: usize = <$Ty as itoa::Integer>::MAX_STR_LEN;
            let _: $Ty = $v;
            assert_eq!(
                int_to_string::<CAP1>(&$v).as_str(),
                impl_itoa::int_to_string::<CAP2>($v).as_str()
            )
        }};
    }

    macro_rules! assert_same_for_ty {
        ($ty:tt ( $($v:expr),* )) => {
            $(assert_same_for_expr!{$ty $v})*
        };
    }

    macro_rules! assert_same {
        ($($Ty:ident $exprs:tt,)+) => {
            $(assert_same_for_ty! {[$Ty] $exprs})+
        };
    }

    #[test]
    fn same() {
        assert_same!(
            u8(0, u8::MAX, 50),
            i8(0, i8::MIN, i8::MAX, -50, 50),
            u16(0, u16::MAX, 520),
            i16(0, i16::MIN, i16::MAX, -520, 520),
            u128(0, u128::MAX, 5201314),
            i128(0, i128::MIN, i128::MAX, 5201314),
        );
    }
}
