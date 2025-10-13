use super::{EitherTextChunks, IterTextChunk};

pub enum Chunk<A, B> {
    A(A),
    B(B),
}

impl<A: AsRef<[u8]>, B: AsRef<[u8]>> AsRef<[u8]> for Chunk<A, B> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::A(v) => v.as_ref(),
            Chunk::B(v) => v.as_ref(),
        }
    }
}

impl<A: IterTextChunk, B: IterTextChunk> IterTextChunk for EitherTextChunks<A, B> {
    type Chunk<'a>
        = Chunk<A::Chunk<'a>, B::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        match self {
            EitherTextChunks::A(this) => this.next_text_chunk().map(Chunk::A),
            EitherTextChunks::B(this) => this.next_text_chunk().map(Chunk::B),
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        match self {
            EitherTextChunks::A(this) => this.bytes_len_hint(),
            EitherTextChunks::B(this) => this.bytes_len_hint(),
        }
    }
}
