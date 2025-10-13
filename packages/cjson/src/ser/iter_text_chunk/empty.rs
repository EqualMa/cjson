use crate::ser::iter_text_chunk::IterTextChunk;

pub enum Chunk {}

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        match *self {}
    }
}

impl IterTextChunk for super::Empty {
    type Chunk<'a>
        = Chunk
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        None
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
