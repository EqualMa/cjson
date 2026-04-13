use core::mem;

use crate::{ser::iter_text_chunk::IterTextChunk, utils::size_hint::SizeHint};

use super::{
    super::traits::{self, Kvs},
    Braced,
};

// TODO: refactor to Grouped<Inner, const OPEN: u8, const CLOSE: u8>
#[derive(Debug)]
enum Inner<Values: IterTextChunk> {
    Init(Values),
    Chunks(Values),
    Finished,
}

pub enum Chunk<T> {
    LeftBrace,
    Inner(T),
    RightBrace,
}

impl<Values: IterTextChunk> Inner<Values> {
    fn next_text_chunk(&mut self) -> Option<Chunk<Values::Chunk<'_>>> {
        match self {
            Inner::Init(_) => {
                *self = Inner::Chunks(match mem::replace(self, Inner::Finished) {
                    Inner::Init(v) => v,
                    _ => unreachable!(),
                });
                Some(Chunk::LeftBrace)
            }
            Inner::Chunks(values) => match values.next_text_chunk() {
                Some(chunk) => Some(Chunk::Inner(chunk)),
                None => Some(Chunk::RightBrace),
            },
            Inner::Finished => None,
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::Inner(chunk) => chunk.as_ref(),
            Chunk::LeftBrace => b"{",
            Chunk::RightBrace => b"}",
        }
    }
}

pub struct TextChunks<Values: IterTextChunk>(Inner<Values>);

impl<Values: IterTextChunk> IterTextChunk for TextChunks<Values> {
    type Chunk<'a>
        = Chunk<Values::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_text_chunk()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let (iter, v) = match &self.0 {
            Inner::Init(iter) => (iter, 2usize),   // `{}`
            Inner::Chunks(iter) => (iter, 1usize), // `}`
            Inner::Finished => return (0, Some(0)),
        };

        (SizeHint(iter.bytes_len_hint()) + v).0
    }
}

impl<T: Kvs> traits::IntoTextChunks for Braced<T> {
    type IntoTextChunks = TextChunks<T::IntoTextChunks>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        TextChunks(Inner::Init(self.0.into_text_chunks()))
    }
}
impl<T: Kvs> traits::sealed::Text for Braced<T> {}
impl<T: Kvs> traits::Text for Braced<T> {}
impl<T: Kvs> traits::sealed::Value for Braced<T> {}
impl<T: Kvs> traits::Value for Braced<T> {}

impl<T: Kvs> traits::sealed::Object for Braced<T> {}
impl<T: Kvs> traits::Object for Braced<T> {
    type IntoKvs = T;

    fn into_kvs(self) -> Self::IntoKvs {
        self.0
    }
}
