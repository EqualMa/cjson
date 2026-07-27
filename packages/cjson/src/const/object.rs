use crate::{
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunkSurroundedWithCompileTime},
    ser::texts,
};

use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub struct EmptyObject;

impl EmptyObject {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        texts::Value::EMPTY_OBJECT
    }
}

mod empty_object;

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

mod non_empty_object;
