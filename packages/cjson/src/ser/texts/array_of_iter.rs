use core::mem;

use polonius_the_crab::{ForLt, PoloniusResult, polonius};

use crate::utils::size_hint::SizeHint;

use super::{
    super::{iter_text_chunk::IterTextChunk, traits},
    ArrayOfIter,
};

enum Inner<I: Iterator> {
    Init(I),
    Iter(I, I::Item),
    Finished,
}

impl<I: Iterator<Item: IterTextChunk>> Inner<I> {
    fn next_chunk(&mut self) -> Option<Chunk<<I::Item as IterTextChunk>::Chunk<'_>>> {
        struct AssignFinished;

        macro_rules! Output {
            ($lt:lifetime) => {
                Option<Chunk<<I::Item as IterTextChunk>::Chunk<$lt>>>
            };
        }

        macro_rules! BorrowingOutput {
            () => {
                ForLt![<'r> = Output!['r]]
            };
        }

        let this = self;

        match polonius::<Self, AssignFinished, BorrowingOutput![]>(this, |this| match this {
            Inner::Init(iter) => PoloniusResult::Borrowing(Some({
                if let Some(item) = iter.next() {
                    let Inner::Init(rest) = mem::replace(this, Inner::Finished) else {
                        unreachable!()
                    };

                    *this = Inner::Iter(rest, item);
                    Chunk::LeftSquareBracket
                } else {
                    *this = Inner::Finished;
                    Chunk::EmptyArray
                }
            })),
            Inner::Iter(rest, cur) => {
                match polonius::<I::Item, Option<I::Item>, BorrowingOutput![]>(
                    cur,
                    |cur: &mut _| -> PoloniusResult<Output!['_], _> {
                        match IterTextChunk::next_text_chunk(cur) {
                            Some(chunk) => PoloniusResult::Borrowing(Some(Chunk::Item(chunk))),
                            v @ None => {
                                drop(v);
                                PoloniusResult::Owned(rest.next())
                            }
                        }
                    },
                ) {
                    PoloniusResult::Borrowing(v) => PoloniusResult::Borrowing(v),
                    PoloniusResult::Owned {
                        value: next_item,
                        input_borrow: cur,
                    } => match next_item {
                        Some(item) => {
                            *cur = item;
                            PoloniusResult::Borrowing(Some(Chunk::Comma))
                        }
                        None => PoloniusResult::Owned(AssignFinished),
                    },
                }
            }
            Inner::Finished => PoloniusResult::Borrowing(None),
        }) {
            PoloniusResult::Borrowing(v) => v,
            PoloniusResult::Owned {
                value: AssignFinished,
                input_borrow: this,
            } => {
                *this = Inner::Finished;
                Some(Chunk::RightSquareBracket)
            }
        }
    }
}

pub enum Chunk<T> {
    LeftSquareBracket,
    Item(T),
    Comma,
    RightSquareBracket,
    EmptyArray,
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::LeftSquareBracket => b"[",
            Chunk::Item(item) => item.as_ref(),
            Chunk::Comma => b",",
            Chunk::RightSquareBracket => b"]",
            Chunk::EmptyArray => b"[]",
        }
    }
}

pub struct Chunks<I: Iterator<Item: traits::Text>>(Inner<MapIntoTextChunks<I>>);

impl<I: Iterator<Item: traits::Text>> IterTextChunk for Chunks<I> {
    type Chunk<'a>
        = Chunk<<<I::Item as traits::IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_chunk()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        let iter = match &self.0 {
            Inner::Init(iter) => {
                // The first `[`
                // Each item yields at least two bytes like `0,` or `0]`
                iter
            }
            Inner::Iter(iter, _) => {
                // current item yields at least one byte `,`
                // each remaining item yields at least two bytes like `0,` or `0]`
                iter
            }
            Inner::Finished => return (0, Some(0)),
        };

        (SizeHint(iter.size_hint()) * 2 + 1).0
    }
}

#[derive(Debug)]
struct MapIntoTextChunks<I: Iterator<Item: traits::IntoTextChunks>> {
    iter: I,
}

/// [`core::iter::Map`]
impl<I: Iterator<Item: traits::IntoTextChunks<IntoTextChunks = B>>, B> Iterator
    for MapIntoTextChunks<I>
{
    type Item = B;

    crate::utils::iter_map::impl_iter_map!(traits::IntoTextChunks::into_text_chunks);
}

impl<I: Iterator<Item: traits::Text>> traits::IntoTextChunks for ArrayOfIter<I> {
    type IntoTextChunks = Chunks<I>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        Chunks(Inner::Init(MapIntoTextChunks { iter: self.0 }))
    }
}
impl<I: Iterator<Item: traits::Text>> traits::sealed::Text for ArrayOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::Text for ArrayOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::sealed::Value for ArrayOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::Value for ArrayOfIter<I> {}

#[derive(Debug, Clone, Copy)]
pub struct CommaSeparatedElementsOfIter<I: Iterator<Item: traits::Text>>(I);
mod elements;

impl<I: Iterator<Item: traits::Text>> traits::sealed::Array for ArrayOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::Array for ArrayOfIter<I> {
    type IntoCommaSeparatedElements = CommaSeparatedElementsOfIter<I>;

    fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
        CommaSeparatedElementsOfIter(self.0)
    }
}

#[cfg(test)]
mod tests;
