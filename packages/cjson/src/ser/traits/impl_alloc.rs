use alloc::{string::String, vec::Vec};

use super::{ConsumeTextChunk, MutConsume};

impl ConsumeTextChunk for String {
    fn consume_text_chunk(&mut self, chunk: &str) {
        self.push_str(chunk)
    }
    fn consume_2_text_chunks(&mut self, chunk1: &str, chunk2: &str) {
        self.reserve(chunk1.len() + chunk2.len());
        self.push_str(chunk1);
        self.push_str(chunk2);
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
    fn consume_2_text_chunks(&mut self, chunk1: &str, chunk2: &str) {
        self.reserve(chunk1.len() + chunk2.len());
        self.extend_from_slice(chunk1.as_bytes());
        self.extend_from_slice(chunk2.as_bytes());
    }

    fn as_mut_consume_text_chunk(&mut self) -> impl ConsumeTextChunk
    where
        Self: Sized,
    {
        MutConsume(self)
    }
}
