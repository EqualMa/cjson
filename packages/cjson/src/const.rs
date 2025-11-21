use core::marker::PhantomData;

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::ser::{ToJson, iter_text_chunk::IterNonLending, texts, traits::IntoTextChunks};

pub struct ConstIntoJson<T>(pub T);

pub struct ConstIntoJsonValueString<T>(pub T);
pub struct ConstAsJsonValueStr<T>(pub T);

pub struct ConstIntoTextChunks<T: ?Sized>(pub T);
pub struct ConstIterTextChunk<T: ?Sized>(pub T);

pub struct BooleanTextChunks(Option<bool>);

impl BooleanTextChunks {
    const fn next_text_chunk() {}
}

#[derive(Debug, RefCastCustom)]
#[repr(transparent)]
pub struct AsRefU8Slice<T>(pub T);

impl<T> AsRefU8Slice<T> {
    #[ref_cast_custom]
    pub(crate) const fn new_ref(s: &T) -> &Self;
}

pub trait HasConstJsonValue {
    const JSON_VALUE: texts::Value<&'static str>;
}

pub struct ConstJsonValue<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> ConstJsonValue<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized> Copy for ConstJsonValue<T> {}
impl<T: ?Sized> Clone for ConstJsonValue<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Default for ConstJsonValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JsonValueString<const LEN: usize>([u8; LEN]);

impl<T: ?Sized + HasConstJsonValue> ConstJsonValue<T> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        T::JSON_VALUE
    }
}

mod ser {
    use core::marker::PhantomData;

    use crate::ser::{
        ToJson,
        iter_text_chunk::IterNonLending,
        traits::{self, IntoTextChunks},
    };

    use super::{ConstJsonValue, HasConstJsonValue};

    pub struct Chunk<T: ?Sized + HasConstJsonValue>(PhantomData<T>);

    impl<T: ?Sized + HasConstJsonValue> AsRef<[u8]> for Chunk<T> {
        fn as_ref(&self) -> &[u8] {
            T::JSON_VALUE.inner().as_bytes()
        }
    }

    impl<T: ?Sized + HasConstJsonValue> ToJson for ConstJsonValue<T> {
        type ToJson<'a>
            = Self
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            *self
        }
    }

    impl<T: ?Sized + HasConstJsonValue> IntoTextChunks for ConstJsonValue<T> {
        type IntoTextChunks = IterNonLending<core::iter::Once<Chunk<T>>>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            IterNonLending(core::iter::once(Chunk(PhantomData)))
        }
    }

    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Value for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Value for ConstJsonValue<T> {}

    mod r#const {}
}

#[cfg(test)]
mod const_tests {
    use super::{ConstIntoJson, ConstIntoTextChunks, ConstIterTextChunk};

    const fn assert_ser_bool(v: bool) {
        let mut chunks = ConstIterTextChunk(
            ConstIntoTextChunks(ConstIntoJson(v).const_into_json()).const_into_text_chunks(),
        );

        let Some(chunk) = chunks.const_next_text_chunk() else {
            panic!()
        };
        let chunk = chunk.as_ref_u8_slice();

        if v {
            assert!(matches!(chunk, b"true"));
        } else {
            assert!(matches!(chunk, b"false"));
        }

        assert!(chunks.const_next_text_chunk().is_none());
    }

    const _: () = {
        assert_ser_bool(true);
        assert_ser_bool(false);
    };
}

pub(crate) mod array_string;
pub(crate) mod json_value_array_str;
