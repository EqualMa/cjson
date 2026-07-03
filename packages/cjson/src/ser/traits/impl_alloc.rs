use alloc::{string::String, vec::Vec};

use super::{ConsumeTextChunk, MutConsume};

impl ConsumeTextChunk for String {
    fn consume_text_chunk(&mut self, chunk: &str) {
        self.push_str(chunk)
    }

    fn as_mut_consume_text_chunk(&mut self) -> impl ConsumeTextChunk
    where
        Self: Sized,
    {
        MutConsume(self)
    }
}

impl ConsumeTextChunk for Vec<u8> {
    fn consume_text_chunk(&mut self, chunk: &str) {
        self.extend_from_slice(chunk.as_bytes())
    }

    fn as_mut_consume_text_chunk(&mut self) -> impl ConsumeTextChunk
    where
        Self: Sized,
    {
        MutConsume(self)
    }
}
