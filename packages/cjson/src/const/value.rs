use crate::{
    ToJson,
    r#const::{CompileTimeChunk, HasConstCompileTimeChunk, RuntimeChunk, State},
    ser::texts,
};

#[derive(Debug, Clone, Copy)]
pub struct Value<C: RuntimeChunk>(C);

impl<C: RuntimeChunk> Value<C> {
    pub const fn new(chunk: C) -> Self {
        const {
            C::PREV_STATE.assert_same(&State::INIT);
            C::NEXT_STATE.assert_eof();
        }
        Self(chunk)
    }

    pub(crate) const fn inner(&self) -> &C {
        &self.0
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> Value<CompileTimeChunk<T>> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        texts::Value::new_without_validation(T::CHUNK.into_inner())
    }
}
