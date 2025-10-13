use core::mem;

use polonius_the_crab::{ForLt, PoloniusResult, polonius};

use crate::{ser::iter_text_chunk::IterTextChunk, utils::size_hint::SizeHint};

use super::{
    super::traits::{self, IntoTextChunks},
    QuotedJsonStringFragment,
};

#[derive(Debug)]
enum Inner<T: IterTextChunk> {
    Start(T),
    Fragments(T),
    End,
}

impl<T: IterTextChunk> Inner<T> {
    fn next(&mut self) -> Option<Chunk<T::Chunk<'_>>> {
        struct AssignEnd;
        match polonius::<Self, AssignEnd, ForLt![Option<Chunk<T::Chunk<'_>>>]>(self, |this| {
            match this {
                Self::Start(_) => PoloniusResult::Borrowing({
                    let Self::Start(frag) = mem::replace(this, Self::End) else {
                        unreachable!()
                    };

                    if frag.bytes_len_hint().1 == Some(0) {
                        // *self = Self::End;
                        Some(Chunk::EmptyString)
                    } else {
                        *this = Self::Fragments(frag);
                        Some(Chunk::QuotationMark)
                    }
                }),
                Self::Fragments(frags) => {
                    if let Some(chunk) = frags.next_text_chunk() {
                        PoloniusResult::Borrowing(Some(Chunk::Chunk(chunk)))
                    } else {
                        PoloniusResult::Owned(AssignEnd)
                    }
                }
                Self::End => PoloniusResult::Borrowing(None),
            }
        }) {
            PoloniusResult::Borrowing(v) => v,
            PoloniusResult::Owned {
                value: AssignEnd,
                input_borrow: this,
            } => {
                *this = Self::End;
                Some(Chunk::QuotationMark)
            }
        }
    }
}

#[derive(Debug)]
pub struct Chunks<T: IterTextChunk>(Inner<T>);
impl<T: IterTextChunk> Chunks<T> {
    const fn new(bytes: T) -> Self {
        Self(Inner::Start(bytes))
    }
}

pub enum Chunk<T> {
    QuotationMark,
    EmptyString,
    Chunk(T),
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::QuotationMark => "\"".as_bytes(),
            Chunk::EmptyString => "\"\"".as_bytes(),
            Chunk::Chunk(v) => v.as_ref(),
        }
    }
}

impl<T: IterTextChunk> IterTextChunk for Chunks<T> {
    type Chunk<'a>
        = Chunk<T::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let (frag, start) = match &self.0 {
            Inner::Start(frag) => (frag, true),
            Inner::Fragments(frag) => (frag, false),
            Inner::End => return (0, Some(0)),
        };

        (SizeHint(frag.bytes_len_hint()) + (if start { 2 } else { 1 })).0
    }
}

impl<T: traits::JsonStringFragment> IntoTextChunks for QuotedJsonStringFragment<T> {
    type IntoTextChunks = Chunks<T::IntoTextChunks>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        Chunks::new(self.0.into_text_chunks())
    }
}

impl<T: traits::JsonStringFragment> traits::sealed::Text for QuotedJsonStringFragment<T> {}
impl<T: traits::JsonStringFragment> traits::Text for QuotedJsonStringFragment<T> {}
impl<T: traits::JsonStringFragment> traits::sealed::Value for QuotedJsonStringFragment<T> {}
impl<T: traits::JsonStringFragment> traits::Value for QuotedJsonStringFragment<T> {}
