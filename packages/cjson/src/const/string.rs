use crate::{
    ToJson,
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunkSurroundedWithCompileTime},
    ser::{ToJsonString, texts},
};

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct JsonStringOfFragments<T: ToJsonString>(pub T);

impl<T: ToJsonString> ToJson for JsonStringOfFragments<T> {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}

impl<T: ToJsonString> ToJsonString for JsonStringOfFragments<T> {
    type ToJsonString<'a>
        = T::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        T::to_json_string(&self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JsonString<C: RuntimeChunkSurroundedWithCompileTime>(Value<C>);

impl<C: RuntimeChunkSurroundedWithCompileTime> JsonString<C> {
    pub const fn new(chunk: Value<C>) -> Self {
        const { () = Self::ASSERT }
        Self(chunk)
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> JsonString<CompileTimeChunk<T>> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        self.0.as_json_value_str()
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJson for JsonString<C> {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJsonString for JsonString<C> {
    type ToJsonString<'a>
        = ser::JsonStringSer<C::ChunksReadyToUngroup<'a>>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        ser::JsonStringSer::from_json_string::<C>(self)
    }
}

mod ser;
