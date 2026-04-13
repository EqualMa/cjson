use crate::{
    ToJson,
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunkSurroundedWithCompileTime},
    ser::{ToJsonArray, texts},
};

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct EmptyArray;

impl EmptyArray {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        texts::Value::EMPTY_ARRAY
    }
}

impl ToJson for EmptyArray {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

impl ToJsonArray for EmptyArray {
    type ToJsonArray<'a>
        = Self
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        Self
    }
}

mod empty_array;

#[derive(Debug, Clone, Copy)]
pub struct ArrayOfItems<T: ToJsonArray>(pub T);

impl<T: ToJsonArray> ToJson for ArrayOfItems<T> {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

impl<T: ToJsonArray> ToJsonArray for ArrayOfItems<T> {
    type ToJsonArray<'a>
        = T::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        T::to_json_array(&self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonEmptyArray<C: RuntimeChunkSurroundedWithCompileTime>(Value<C>);

impl<C: RuntimeChunkSurroundedWithCompileTime> NonEmptyArray<C> {
    pub const fn new(chunk: Value<C>) -> Self {
        const { () = Self::ASSERT }
        Self(chunk)
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> NonEmptyArray<CompileTimeChunk<T>> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        self.0.as_json_value_str()
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJson for NonEmptyArray<C> {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJsonArray for NonEmptyArray<C> {
    type ToJsonArray<'a>
        = non_empty_array::NonEmptyArraySer<C::ChunksReadyToUngroup<'a>>
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        non_empty_array::NonEmptyArraySer::from_non_empty_array::<C>(self)
    }
}

mod non_empty_array;
