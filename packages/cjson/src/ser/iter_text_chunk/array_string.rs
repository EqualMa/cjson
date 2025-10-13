use arrayvec::ArrayString;

use super::IterTextChunk;

pub struct Chunk<const CAP: usize>(ArrayString<CAP>);

impl<const CAP: usize> AsRef<[u8]> for Chunk<CAP> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<const CAP: usize> IterTextChunk for ArrayString<CAP> {
    type Chunk<'a>
        = Chunk<CAP>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        if self.is_empty() {
            None
        } else {
            let chunk = *self;
            self.clear();
            Some(Chunk(chunk))
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
