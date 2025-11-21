use arrayvec::ArrayString;

use crate::{
    ser::{ToJson, texts},
    utils::impl_many,
};

#[cfg(any(test, not(feature = "itoa")))]
mod impl_display;
#[cfg(feature = "itoa")]
mod impl_itoa;

#[cfg(not(feature = "itoa"))]
use self::impl_display as imp;
#[cfg(feature = "itoa")]
use self::impl_itoa as imp;

impl_many!(
    impl<__> ToJson
        for each_of![
            i8, i16, i32, i64, isize, i128, //
            u8, u16, u32, u64, usize, u128,
        ]
    {
        type ToJson<'a>
            = texts::Number<ArrayString<{ <Self as imp::Integer>::MAX_STR_LEN }>>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            let ret = imp::int_to_string(self);

            texts::Number::new_without_validation(ret)
        }
    }
);

mod r#const;
