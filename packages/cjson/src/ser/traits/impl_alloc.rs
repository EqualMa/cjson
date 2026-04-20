use alloc::{string::String, vec::Vec};

use super::ConsumeTextChunk;

impl ConsumeTextChunk for String {
    fn consume_text_chunk(&mut self, chunk: &str) {
        self.push_str(chunk)
    }
}

impl ConsumeTextChunk for Vec<u8> {
    fn consume_text_chunk(&mut self, chunk: &str) {
        self.extend_from_slice(chunk.as_bytes())
    }
}
