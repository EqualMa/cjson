use crate::{
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunkSurroundedWithCompileTime},
    ser::texts,
};

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct EmptyArray;

impl EmptyArray {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        texts::Value::EMPTY_ARRAY
    }
}

mod empty_array;

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

mod non_empty_array;
