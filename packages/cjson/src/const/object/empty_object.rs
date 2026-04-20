use crate::ser::{
    iter_text_chunk::HasConstChunk,
    texts,
    traits::{self, IntoTextChunks, proxy_IntoTextChunks},
};

use super::EmptyObject;

pub enum Chunk {}

impl HasConstChunk for Chunk {
    const CHUNK: &'static str = texts::Value::EMPTY_OBJECT.inner();
}

impl IntoTextChunks for EmptyObject {
    proxy_IntoTextChunks!(|self| -> texts::ConstChunk<Chunk> { texts::ConstChunk::DEFAULT });
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
