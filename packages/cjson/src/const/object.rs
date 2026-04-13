use crate::{
    ToJson,
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunkSurroundedWithCompileTime},
    ser::{ToJsonObject, texts},
};

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct EmptyObject;

impl EmptyObject {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        texts::Value::EMPTY_OBJECT
    }
}

impl ToJson for EmptyObject {
    type ToJson<'a>
        = <Self as ToJsonObject>::ToJsonObject<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_object(self)
    }
}

impl ToJsonObject for EmptyObject {
    type ToJsonObject<'a>
        = Self
    where
        Self: 'a;

    fn to_json_object(&self) -> Self::ToJsonObject<'_> {
        Self
    }
}

mod empty_object;

#[derive(Debug, Clone, Copy)]
pub struct ObjectOfKvs<T: ToJsonObject>(pub T);

impl<T: ToJsonObject> ToJson for ObjectOfKvs<T> {
    type ToJson<'a>
        = <Self as ToJsonObject>::ToJsonObject<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_object(self)
    }
}

impl<T: ToJsonObject> ToJsonObject for ObjectOfKvs<T> {
    type ToJsonObject<'a>
        = T::ToJsonObject<'a>
    where
        Self: 'a;

    fn to_json_object(&self) -> Self::ToJsonObject<'_> {
        T::to_json_object(&self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonEmptyObject<C: RuntimeChunkSurroundedWithCompileTime>(Value<C>);

impl<C: RuntimeChunkSurroundedWithCompileTime> NonEmptyObject<C> {
    pub const fn new(chunk: Value<C>) -> Self {
        const { () = Self::ASSERT }
        Self(chunk)
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> NonEmptyObject<CompileTimeChunk<T>> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        self.0.as_json_value_str()
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJson for NonEmptyObject<C> {
    type ToJson<'a>
        = <Self as ToJsonObject>::ToJsonObject<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_object(self)
    }
}

impl<C: RuntimeChunkSurroundedWithCompileTime> ToJsonObject for NonEmptyObject<C> {
    type ToJsonObject<'a>
        = non_empty_object::NonEmptyObjectSer<C::ChunksReadyToUngroup<'a>>
    where
        Self: 'a;

    fn to_json_object(&self) -> Self::ToJsonObject<'_> {
        non_empty_object::NonEmptyObjectSer::from_non_empty_object::<C>(self)
    }
}

mod non_empty_object;
