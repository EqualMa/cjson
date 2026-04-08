use crate::{
    ToJson,
    r#const::{RuntimeChunk, State},
    ser::texts,
};

#[derive(Debug, Clone, Copy)]
pub struct Value<C: RuntimeChunk>(C);

impl<C: RuntimeChunk> Value<C> {
    pub const fn new(chunk: C) -> Self {
        const {
            C::PREV_STATE.assert_same(State::INIT);
            C::NEXT_STATE.assert_same(State::EOF);
        }
        Self(chunk)
    }

    pub(crate) const fn inner(&self) -> &C {
        &self.0
    }
}

impl<C: RuntimeChunk> ToJson for Value<C> {
    type ToJson<'a>
        = texts::Value<C::ToIntoTextChunks<'a>>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        texts::Value::new_without_validation(self.inner().to_into_text_chunks())
    }
}

#[cfg(todo)]
impl<C: RuntimeChunk> ToJson for Value<C> {
    type ToJson<'a>
        = ser::ValueSer<'a, C>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        ser::ValueSer(self)
    }
}

#[cfg(todo)]
mod ser;
