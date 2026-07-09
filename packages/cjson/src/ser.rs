pub use self::consumers::{
    ConsumeChainedArrays, ConsumeJson, ConsumeJsonChunks, ConsumeJsonText, Consumed, json_kinds,
};

use json_kinds::JsonKind;

mod consumers;
pub mod iter_text_chunk;
pub mod texts;
pub mod traits;
pub mod values;

pub mod exts;

pub trait IntoJson {
    type JsonKind: JsonKind;
    fn json_provide_into<W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>>(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W>;

    /// If implemented as `true`, chaining with another json value is optimized.
    /// Note that it is always correct to implement as `false`.
    ///
    /// Wrong implementations will not affect json validity.
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool;
}

pub trait ToJson {
    type ToJson<'a>: traits::Text
    where
        Self: 'a;
    fn to_json(&self) -> Self::ToJson<'_>;
}

impl<'this, T: ?Sized + ToJson> ToJson for &'this T {
    type ToJson<'a>
        = T::ToJson<'this>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        T::to_json(self)
    }
}

pub trait ToJsonString {
    type ToJsonString<'a>: traits::JsonString
    where
        Self: 'a;
    fn to_json_string(&self) -> Self::ToJsonString<'_>;
}

impl<'this, T: ?Sized + ToJsonString> ToJsonString for &'this T {
    type ToJsonString<'a>
        = T::ToJsonString<'this>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        T::to_json_string(self)
    }
}

pub trait ToJsonArray: ToJson {
    type ToJsonArray<'a>: traits::Array
    where
        Self: 'a;
    fn to_json_array(&self) -> Self::ToJsonArray<'_>;
}

impl<'this, T: ?Sized + ToJsonArray> ToJsonArray for &'this T {
    type ToJsonArray<'a>
        = T::ToJsonArray<'this>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        T::to_json_array(self)
    }
}

pub trait ToJsonObject: ToJson {
    type ToJsonObject<'a>: traits::Object
    where
        Self: 'a;
    fn to_json_object(&self) -> Self::ToJsonObject<'_>;
}

impl<'this, T: ?Sized + ToJsonObject> ToJsonObject for &'this T {
    type ToJsonObject<'a>
        = T::ToJsonObject<'this>
    where
        Self: 'a;

    fn to_json_object(&self) -> Self::ToJsonObject<'_> {
        T::to_json_object(self)
    }
}

mod bool;
mod int;
mod string;

mod slice;

mod tuple;

#[cfg(feature = "alloc")]
mod alloc;

pub fn write_json_text(value: impl IntoJson, w: impl traits::ConsumeTextChunk) {
    let Consumed { .. } = value.json_provide_into(consumers::ConsumeJsonText(w));
}
