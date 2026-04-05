use core::mem;

use crate::{ser::iter_text_chunk::IterTextChunk, utils::size_hint::SizeHint};

use super::{
    super::traits::{self, EmptyOrCommaSeparatedElements},
    Bracketed,
};

#[derive(Debug)]
enum Inner<Values: IterTextChunk> {
    Init(Values),
    Chunks(Values),
    Finished,
}

pub enum Chunk<T> {
    LeftSquareBracket,
    CommaSeparatedValuesChunk(T),
    RightSquareBracket,
}

impl<Values: IterTextChunk> Inner<Values> {
    fn next_text_chunk(&mut self) -> Option<Chunk<Values::Chunk<'_>>> {
        match self {
            Inner::Init(_) => {
                *self = Inner::Chunks(match mem::replace(self, Inner::Finished) {
                    Inner::Init(v) => v,
                    _ => unreachable!(),
                });
                Some(Chunk::LeftSquareBracket)
            }
            Inner::Chunks(values) => match values.next_text_chunk() {
                Some(chunk) => Some(Chunk::CommaSeparatedValuesChunk(chunk)),
                None => Some(Chunk::RightSquareBracket),
            },
            Inner::Finished => None,
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::CommaSeparatedValuesChunk(chunk) => chunk.as_ref(),
            Chunk::LeftSquareBracket => b"[",
            Chunk::RightSquareBracket => b"]",
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
            Inner::Init(iter) => (iter, 2usize),   // `[]`
            Inner::Chunks(iter) => (iter, 1usize), // `]`
            Inner::Finished => return (0, Some(0)),
        };

        (SizeHint(iter.bytes_len_hint()) + v).0
    }
}

impl<Values: EmptyOrCommaSeparatedElements> traits::IntoTextChunks for Bracketed<Values> {
    type IntoTextChunks = TextChunks<Values::IntoTextChunks>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        TextChunks(Inner::Init(self.0.into_text_chunks()))
    }
}
impl<Values: EmptyOrCommaSeparatedElements> traits::sealed::Text for Bracketed<Values> {}
impl<Values: EmptyOrCommaSeparatedElements> traits::Text for Bracketed<Values> {}
impl<Values: EmptyOrCommaSeparatedElements> traits::sealed::Value for Bracketed<Values> {}
impl<Values: EmptyOrCommaSeparatedElements> traits::Value for Bracketed<Values> {}

impl<Values: EmptyOrCommaSeparatedElements> traits::sealed::Array for Bracketed<Values> {}
impl<Values: EmptyOrCommaSeparatedElements> traits::Array for Bracketed<Values> {
    type IntoCommaSeparatedElements = Values;

    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        self.0
    }
}
