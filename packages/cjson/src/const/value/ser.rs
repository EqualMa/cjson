use crate::{
    r#const::RuntimeChunk,
    ser::traits::{self, IntoTextChunks},
};

use super::Value;

pub struct ValueSer<'a, C: RuntimeChunk>(pub(crate) &'a Value<C>);

impl<'a, C: RuntimeChunk> IntoTextChunks for ValueSer<'a, C> {
    type IntoTextChunks = <C::ToIntoTextChunks<'a> as IntoTextChunks>::IntoTextChunks;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        self.0.inner().to_into_text_chunks().into_text_chunks()
    }

    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.0
            .inner()
            .to_into_text_chunks()
            ._private_into_text_chunks_vec()
    }
}

impl<C: RuntimeChunk> traits::sealed::Text for ValueSer<'_, C> {}
impl<C: RuntimeChunk> traits::Text for ValueSer<'_, C> {}
impl<C: RuntimeChunk> traits::sealed::Value for ValueSer<'_, C> {}
impl<C: RuntimeChunk> traits::Value for ValueSer<'_, C> {}
