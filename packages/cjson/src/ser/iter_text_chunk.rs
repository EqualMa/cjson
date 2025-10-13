pub trait IterTextChunk {
    type Chunk<'a>: AsRef<[u8]>
    where
        Self: 'a;
    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>>;

    /// See [`core::iter::Iterator::size_hint`]
    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

mod array_string;
mod str;
mod u8_slice;

pub enum EitherTextChunks<A, B> {
    A(A),
    B(B),
}

mod either;

pub struct IterNonLending<I: Iterator<Item: AsRef<[u8]>>>(pub I);

impl<I: Iterator<Item: AsRef<[u8]>>> IterTextChunk for IterNonLending<I> {
    type Chunk<'a>
        = I::Item
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next()
    }
}

/// [`core::iter::Chain`]
#[derive(Debug, Clone, Copy)]
pub struct Chain<A: IterTextChunk, B: IterTextChunk>(Option<(Option<A>, B)>);
mod chain;

impl<A: IterTextChunk, B: IterTextChunk> Chain<A, B> {
    pub const fn new(a: A, b: B) -> Self {
        Self(Some((Some(a), b)))
    }
}
