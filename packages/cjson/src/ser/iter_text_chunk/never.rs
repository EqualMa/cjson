use super::{IterTextChunk, NeverTextChunk};

impl IterTextChunk for NeverTextChunk {
    type Chunk<'a>
        = super::empty::Chunk
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        match *self {}
    }
}
