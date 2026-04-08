use crate::ser::{
    iter_text_chunk::{ConstChunk, HasConstChunk, IterTextChunk},
    texts,
    traits::{self, IntoTextChunks},
};

use super::EmptyArray;

pub enum Chunk {}

impl HasConstChunk for Chunk {
    const CHUNK: &'static str = "[]";
}

impl IntoTextChunks for EmptyArray {
    type IntoTextChunks = ConstChunk<Chunk>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        ConstChunk::DEFAULT
    }

    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.into_text_chunks()._private_collect_into_vec()
    }
}

impl traits::sealed::Text for EmptyArray {}
impl traits::Text for EmptyArray {}
impl traits::sealed::Value for EmptyArray {}
impl traits::Value for EmptyArray {}
impl traits::sealed::Array for EmptyArray {}
impl traits::Array for EmptyArray {
    type IntoCommaSeparatedElements = texts::Empty;

    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        texts::Empty
    }
}
