use super::IterTextChunk;

impl IterTextChunk for &[u8] {
    type Chunk<'a>
        = Self
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        if self.is_empty() {
            None
        } else {
            let this = *self;
            *self = &[];
            Some(this)
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
