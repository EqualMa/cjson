use crate::ser::{
    iter_text_chunk::{ConstChunk, HasConstChunk, IterTextChunk},
    texts,
    traits::{self, IntoTextChunks},
};

use super::EmptyObject;

pub enum Chunk {}

impl HasConstChunk for Chunk {
    const CHUNK: &'static str = texts::Value::EMPTY_OBJECT.inner();
}

impl IntoTextChunks for EmptyObject {
    type IntoTextChunks = ConstChunk<Chunk>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        ConstChunk::DEFAULT
    }

    fn _private_into_text_chunks_vec(self) -> alloc::vec::Vec<u8> {
        self.into_text_chunks()._private_collect_into_vec()
    }
}

impl traits::sealed::Text for EmptyObject {}
impl traits::Text for EmptyObject {}
impl traits::sealed::Value for EmptyObject {}
impl traits::Value for EmptyObject {}
impl traits::sealed::Object for EmptyObject {}
impl traits::Object for EmptyObject {
    type IntoKvs = texts::Empty;

    fn into_kvs(self) -> Self::IntoKvs {
        texts::Empty
    }
}
