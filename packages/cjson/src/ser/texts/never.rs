use crate::{
    ser::{
        iter_text_chunk::NeverTextChunk,
        traits::{self, IntoTextChunks},
    },
    values::Never,
};

impl IntoTextChunks for Never {
    type IntoTextChunks = NeverTextChunk;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        match self {}
    }

    #[cfg(feature = "alloc")]
    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        match self {}
    }
}

impl traits::sealed::Text for Never {}
impl traits::Text for Never {}
impl traits::sealed::Value for Never {}
impl traits::Value for Never {}
impl traits::sealed::JsonStringFragment for Never {}
impl traits::JsonStringFragment for Never {}
