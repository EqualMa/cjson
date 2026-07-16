use crate::{
    r#const::array_string::ArrayString,
    ser::{
        IntoJson, ToJson, ToJsonByCopyIntoJson, json_kinds, texts,
        traits::{self, IntoTextChunks},
    },
    utils::impl_many,
};

use super::Finite;

const SIZE: usize = core::mem::size_of::<ryu::Buffer>();

impl_many!({
    {
        {
            use f64 as Float;
        }
        {
            use f32 as Float;
        }
    }

    impl IntoJson for Finite<Float> {
        type JsonKind = json_kinds::AnyValue;
        fn json_provide_into<
            W: crate::ser::ConsumeJson<
                    ConsumeJsonKind: json_kinds::JsonKind<Contains<Self::JsonKind> = ()>,
                >,
        >(
            self,
            w: W,
        ) -> crate::ser::Consumed<Self::JsonKind, W> {
            w.consume_any_value(texts::Value::new_without_validation(self), ())
        }
        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    }

    impl ToJsonByCopyIntoJson for Finite<Float> {}

    impl ToJson for Finite<Float> {
        type ToJson<'a>
            = Self
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            *self
        }
    }

    impl traits::sealed::Text for Finite<Float> {}
    impl traits::Text for Finite<Float> {}
    impl traits::sealed::Value for Finite<Float> {}
    impl traits::Value for Finite<Float> {}

    impl IntoTextChunks for Finite<Float> {
        type IntoTextChunks = ArrayString<u8, SIZE>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            let mut buf = ryu::Buffer::new();
            let s = buf.format_finite(self.0);
            ArrayString::from_str(s)
        }

        fn write_into<W: ?Sized + crate::ser::traits::ConsumeTextChunk>(self, w: &mut W) {
            let mut buf = ryu::Buffer::new();
            let s = buf.format_finite(self.0);
            w.consume_text_chunk(s)
        }

        fn try_write_into<W: ?Sized + crate::ser::traits::TryConsumeTextChunk>(
            self,
            w: &mut W,
        ) -> Result<(), W::Err> {
            let mut buf = ryu::Buffer::new();
            let s = buf.format_finite(self.0);
            w.try_consume_text_chunk(s)
        }
    }
});

mod r#const;
