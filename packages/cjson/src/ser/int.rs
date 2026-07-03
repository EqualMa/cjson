use core::mem::MaybeUninit;

use crate::{
    r#const::array_string::ArrayString,
    ser::{
        ConsumeJson, Consumed, IntoJson, ToJson,
        json_kinds::{self, JsonKind},
        texts,
        traits::{ConsumeTextChunk, IntoTextChunks, TryConsumeTextChunk},
    },
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
            = texts::Number<Self>
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            texts::Number::new_without_validation(*self)
        }
    }
);

impl_many!(
    impl<__> IntoJson
        for each_of![
            i8, i16, i32, i64, isize, i128, //
            u8, u16, u32, u64, usize, u128,
        ]
    {
        type JsonKind = json_kinds::AnyValue;
        fn json_provide_into<
            W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
        >(
            self,
            w: W,
        ) -> Consumed<Self::JsonKind, W> {
            w.consume_any_value(texts::Value::new_without_validation(self), ())
        }

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    }
);

impl_many!(
    impl<__> IntoTextChunks
        for each_of![
            i8, i16, i32, i64, isize, i128, //
            u8, u16, u32, u64, usize, u128,
        ]
    {
        type IntoTextChunks = ArrayString<u8, { <Self as imp::Integer>::MAX_STR_LEN }>;
        fn into_text_chunks(self) -> Self::IntoTextChunks {
            imp::int_to_string(self)
        }

        fn write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
            w.consume_text_chunk(imp::format::<{ <Self as imp::Integer>::MAX_STR_LEN }>(
                &mut MaybeUninit::uninit(),
                self,
            ))
        }

        fn try_write_into<W: ?Sized + TryConsumeTextChunk>(self, w: &mut W) -> Result<(), W::Err> {
            w.try_consume_text_chunk(imp::format::<{ <Self as imp::Integer>::MAX_STR_LEN }>(
                &mut MaybeUninit::uninit(),
                self,
            ))
        }
    }
);

mod r#const;
