use crate::ser::{
    iter_text_chunk::HasConstChunk,
    texts::{self, ConstChunk},
    traits::{self, IntoTextChunks, proxy_IntoTextChunks},
};

use super::EmptyArray;

pub enum Chunk {}

impl HasConstChunk for Chunk {
    const CHUNK: &'static str = texts::Value::EMPTY_ARRAY.inner();
}

impl IntoTextChunks for EmptyArray {
    proxy_IntoTextChunks!(|self| -> ConstChunk<Chunk> { ConstChunk::DEFAULT });
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
