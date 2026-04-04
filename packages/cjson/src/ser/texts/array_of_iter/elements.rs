use polonius_the_crab::{ForLt, PoloniusResult, polonius};

use crate::{
    ser::{
        iter_text_chunk::IterTextChunk,
        traits::{self, IntoTextChunks},
    },
    utils::size_hint::SizeHint,
};

use super::{CommaSeparatedElementsOfIter, MapIntoTextChunks};

struct Inner<I: Iterator<Item: IterTextChunk>>(Option<(Option<I::Item>, I)>);

impl<I: Iterator<Item: IterTextChunk + core::fmt::Debug> + core::fmt::Debug> Inner<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>, name: &str) -> core::fmt::Result {
        f.debug_tuple(name).field(&self.0).finish()
    }
}

pub enum Chunk<T> {
    Comma,
    Chunk(T),
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Chunk<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Chunk::Comma => b",",
            Chunk::Chunk(v) => v.as_ref(),
        }
    }
}
impl<I: Iterator<Item: IterTextChunk>> Inner<I> {
    fn next_chunk<const PREPEND_LEADING_COMMA: bool>(
        &mut self,
    ) -> Option<Chunk<<I::Item as IterTextChunk>::Chunk<'_>>> {
        struct AssignEnd;

        macro_rules! BorrowingOutput {
            () => {
                ForLt![Option<Chunk<<I::Item as IterTextChunk>::Chunk<'_>>>]
            };
        }

        match polonius::<Self, AssignEnd, BorrowingOutput![]>(self, |this| {
            let (cur, iter) = match &mut this.0 {
                Some((cur_opt, iter)) => match cur_opt {
                    Some(cur) => (cur, iter),
                    None => {
                        *cur_opt = iter.next();
                        match cur_opt {
                            Some(cur) => {
                                if PREPEND_LEADING_COMMA {
                                    return PoloniusResult::Borrowing(Some(Chunk::Comma));
                                } else {
                                    (cur, iter)
                                }
                            }
                            None => return PoloniusResult::Owned(AssignEnd),
                        }
                    }
                },
                None => return PoloniusResult::Borrowing(None),
            };

            struct NextItem<T>(Option<T>);

            match polonius::<I::Item, (), BorrowingOutput![]>(cur, |cur| {
                match cur.next_text_chunk() {
                    Some(v) => PoloniusResult::Borrowing(Some(Chunk::Chunk(v))),
                    None => PoloniusResult::Owned(()),
                }
            }) {
                PoloniusResult::Borrowing(v) => PoloniusResult::Borrowing(v),
                PoloniusResult::Owned {
                    value: (),
                    input_borrow: cur,
                } => match iter.next() {
                    Some(item) => {
                        *cur = item;
                        PoloniusResult::Borrowing(Some(Chunk::Comma))
                    }
                    None => PoloniusResult::Owned(AssignEnd),
                },
            }
        }) {
            PoloniusResult::Borrowing(v) => v,
            PoloniusResult::Owned {
                value: AssignEnd,
                input_borrow: this,
            } => {
                this.0 = None;
                None
            }
        }
    }

    fn bytes_len_hint<const PREPEND_LEADING_COMMA: bool>(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Some((cur_opt, iter)) => {
                match cur_opt {
                    Some(cur) => {
                        (SizeHint(cur.bytes_len_hint()) + (SizeHint(iter.size_hint()) * 2)).0
                    }
                    // not started
                    None => {
                        if PREPEND_LEADING_COMMA {
                            (SizeHint(iter.size_hint()) * 2).0
                        } else {
                            // SizeHint(iter.size_hint()) * 2 - 1

                            const MAX_V: usize = (usize::MAX - 1) / 2 + 1;
                            const fn checked_mul2sub1(v: usize) -> Option<usize> {
                                match v {
                                    1..MAX_V => Some(v * 2 - 1),
                                    MAX_V => Some(usize::MAX),
                                    _ => None,
                                }
                            }

                            const _: () = {
                                assert!(usize::BITS <= u128::BITS);
                                assert!((MAX_V as u128) * 2 - 1 == usize::MAX as u128);

                                assert!(matches!(checked_mul2sub1(MAX_V), Some(usize::MAX)));
                                assert!(matches!(checked_mul2sub1(0), None));
                                assert!(matches!(checked_mul2sub1(1), Some(1)));
                            };

                            const fn zero_or_mul2sub1(v: usize) -> Option<usize> {
                                if v == 0 { Some(0) } else { checked_mul2sub1(v) }
                            }

                            let (lower, upper) = iter.size_hint();

                            (
                                zero_or_mul2sub1(lower).unwrap_or(
                                    // None means lower is too large.
                                    usize::MAX,
                                ),
                                upper.and_then(zero_or_mul2sub1),
                            )
                        }
                    }
                }
            }
            None => (0, Some(0)),
        }
    }

    const fn new(iter: I) -> Self {
        Inner(Some((None, iter)))
    }
}

impl<I: Iterator<Item: traits::Text>> IterTextChunk for Chunks<I> {
    type Chunk<'a>
        = Chunk<<<I::Item as IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_chunk::<false>()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        self.0.bytes_len_hint::<false>()
    }
}

pub struct Chunks<I: Iterator<Item: traits::Text>>(Inner<MapIntoTextChunks<I>>);

impl<I: Iterator<Item: traits::Text> + core::fmt::Debug> core::fmt::Debug for Chunks<I>
where
    <I::Item as IntoTextChunks>::IntoTextChunks: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f, "Chunks")
    }
}

impl<I: Iterator<Item: traits::Text>> IntoTextChunks for CommaSeparatedElementsOfIter<I> {
    type IntoTextChunks = Chunks<I>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        Chunks(Inner::new(MapIntoTextChunks { iter: self.0 }))
    }
}

impl<I: Iterator<Item: traits::Text>> traits::sealed::Text for CommaSeparatedElementsOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::Text for CommaSeparatedElementsOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::sealed::Value for CommaSeparatedElementsOfIter<I> {}
impl<I: Iterator<Item: traits::Text>> traits::Value for CommaSeparatedElementsOfIter<I> {}

impl<I: Iterator<Item: traits::Text>> traits::sealed::EmptyOrCommaSeparatedElements
    for CommaSeparatedElementsOfIter<I>
{
}
impl<I: Iterator<Item: traits::Text>> traits::EmptyOrCommaSeparatedElements
    for CommaSeparatedElementsOfIter<I>
{
    type PrependLeadingCommaIfNotEmpty = PrependLeadingCommaIfNotEmpty<I>;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        PrependLeadingCommaIfNotEmpty(self.0)
    }

    type AppendTrailingCommaIfNotEmpty = AppendTrailingCommaIfNotEmpty<I>;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
        AppendTrailingCommaIfNotEmpty(self.0)
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> = ChainWithComma<I, Other>;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other> {
        ChainWithComma(self.0, other)
    }
}

pub struct PrependLeadingCommaIfNotEmptyChunks<I: Iterator<Item: traits::Text>>(
    Inner<MapIntoTextChunks<I>>,
);

impl<I: Iterator<Item: traits::Text> + core::fmt::Debug> core::fmt::Debug
    for PrependLeadingCommaIfNotEmptyChunks<I>
where
    <I::Item as IntoTextChunks>::IntoTextChunks: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f, "PrependLeadingCommaIfNotEmptyChunks")
    }
}

impl<I: Iterator<Item: traits::Text>> IterTextChunk for PrependLeadingCommaIfNotEmptyChunks<I> {
    type Chunk<'a>
        = Chunk<<<I::Item as IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_chunk::<true>()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        self.0.bytes_len_hint::<true>()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrependLeadingCommaIfNotEmpty<I: Iterator<Item: traits::Text>>(I);

impl<I: Iterator<Item: traits::Text>> IntoTextChunks for PrependLeadingCommaIfNotEmpty<I> {
    type IntoTextChunks = PrependLeadingCommaIfNotEmptyChunks<I>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        PrependLeadingCommaIfNotEmptyChunks(Inner::new(MapIntoTextChunks { iter: self.0 }))
    }
}

impl<I: Iterator<Item: traits::Text>> traits::sealed::EmptyOrLeadingCommaWithCommaSeparatedElements
    for PrependLeadingCommaIfNotEmpty<I>
{
}
impl<I: Iterator<Item: traits::Text>> traits::EmptyOrLeadingCommaWithCommaSeparatedElements
    for PrependLeadingCommaIfNotEmpty<I>
{
}

struct InnerAppendTrailingComma<I: Iterator<Item: IterTextChunk>>(Inner<I>);

pub struct AppendTrailingCommaIfNotEmptyChunks<I: Iterator<Item: traits::Text>>(
    InnerAppendTrailingComma<MapIntoTextChunks<I>>,
);

impl<I: Iterator<Item: IterTextChunk>> InnerAppendTrailingComma<I> {
    fn next_text_chunk(&mut self) -> Option<Chunk<<I::Item as IterTextChunk>::Chunk<'_>>> {
        enum End {
            WithTrailingComma,
            Empty,
        }

        macro_rules! BorrowingOutput {
            () => {
                ForLt![Option<Chunk<<I::Item as IterTextChunk>::Chunk<'_>>>]
            };
        }

        match polonius::<Inner<I>, End, BorrowingOutput![]>(&mut self.0, |this| {
            let (cur, iter) = match &mut this.0 {
                Some((cur_opt, iter)) => match cur_opt {
                    Some(cur) => (cur, iter),
                    None => {
                        // first run
                        *cur_opt = iter.next();
                        match cur_opt {
                            Some(cur) => (cur, iter),
                            None => return PoloniusResult::Owned(End::Empty),
                        }
                    }
                },
                None => return PoloniusResult::Borrowing(None),
            };

            struct NextItem<T>(Option<T>);

            match polonius::<I::Item, (), BorrowingOutput![]>(cur, |cur| {
                match cur.next_text_chunk() {
                    Some(v) => PoloniusResult::Borrowing(Some(Chunk::Chunk(v))),
                    None => PoloniusResult::Owned(()),
                }
            }) {
                PoloniusResult::Borrowing(v) => PoloniusResult::Borrowing(v),
                PoloniusResult::Owned {
                    value: (),
                    input_borrow: cur,
                } => match iter.next() {
                    Some(item) => {
                        *cur = item;
                        PoloniusResult::Borrowing(Some(Chunk::Comma))
                    }
                    None => PoloniusResult::Owned(End::WithTrailingComma),
                },
            }
        }) {
            PoloniusResult::Borrowing(v) => v,
            PoloniusResult::Owned {
                value: end,
                input_borrow: this,
            } => {
                this.0 = None;

                match end {
                    End::WithTrailingComma => Some(Chunk::Comma),
                    End::Empty => None,
                }
            }
        }
    }
    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        match &self.0.0 {
            Some((cur_opt, iter)) => {
                match cur_opt {
                    Some(cur) => {
                        (SizeHint(cur.bytes_len_hint()) + 1 + (SizeHint(iter.size_hint()) * 2)).0
                    }
                    // not started
                    None => (SizeHint(iter.size_hint()) * 2).0,
                }
            }
            None => (0, Some(0)),
        }
    }
}

impl<I: Iterator<Item: traits::Text> + core::fmt::Debug> core::fmt::Debug
    for AppendTrailingCommaIfNotEmptyChunks<I>
where
    <I::Item as IntoTextChunks>::IntoTextChunks: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.0.fmt(f, "AppendTrailingCommaIfNotEmptyChunks")
    }
}

impl<I: Iterator<Item: traits::Text>> IterTextChunk for AppendTrailingCommaIfNotEmptyChunks<I> {
    type Chunk<'a>
        = Chunk<<<I::Item as IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_text_chunk()
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        self.0.bytes_len_hint()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AppendTrailingCommaIfNotEmpty<I: Iterator<Item: traits::Text>>(I);

impl<I: Iterator<Item: traits::Text>> IntoTextChunks for AppendTrailingCommaIfNotEmpty<I> {
    type IntoTextChunks = AppendTrailingCommaIfNotEmptyChunks<I>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        AppendTrailingCommaIfNotEmptyChunks(InnerAppendTrailingComma(Inner::new(
            MapIntoTextChunks { iter: self.0 },
        )))
    }
}

impl<I: Iterator<Item: traits::Text>> traits::sealed::EmptyOrCommaSeparatedElementsWithTrailingComma
    for AppendTrailingCommaIfNotEmpty<I>
{
}
impl<I: Iterator<Item: traits::Text>> traits::EmptyOrCommaSeparatedElementsWithTrailingComma
    for AppendTrailingCommaIfNotEmpty<I>
{
}

pub struct ChainWithComma<
    I: Iterator<Item: traits::Text>,
    Other: traits::EmptyOrCommaSeparatedElements,
>(I, Other);

impl<I: Iterator<Item: traits::Text>, Other: traits::EmptyOrCommaSeparatedElements> IntoTextChunks
    for ChainWithComma<I, Other>
{
    type IntoTextChunks = ChainWithCommaChunks<I, Other>;

    fn into_text_chunks(self) -> Self::IntoTextChunks {
        ChainWithCommaChunks::new(self.0, self.1)
    }
}

impl<I: Iterator<Item: traits::Text>, Other: traits::EmptyOrCommaSeparatedElements>
    traits::sealed::EmptyOrCommaSeparatedElements for ChainWithComma<I, Other>
{
}

impl<I: Iterator<Item: traits::Text>, ThisOther: traits::EmptyOrCommaSeparatedElements>
    traits::EmptyOrCommaSeparatedElements for ChainWithComma<I, ThisOther>
{
    type PrependLeadingCommaIfNotEmpty = super::super::Chain<
        PrependLeadingCommaIfNotEmpty<I>,
        ThisOther::PrependLeadingCommaIfNotEmpty,
    >;

    fn prepend_leading_comma_if_not_empty(self) -> Self::PrependLeadingCommaIfNotEmpty {
        super::super::Chain(
            PrependLeadingCommaIfNotEmpty(self.0),
            self.1.prepend_leading_comma_if_not_empty(),
        )
    }

    type AppendTrailingCommaIfNotEmpty = super::super::Chain<
        AppendTrailingCommaIfNotEmpty<I>,
        ThisOther::AppendTrailingCommaIfNotEmpty,
    >;

    fn append_trailing_comma_if_not_empty(self) -> Self::AppendTrailingCommaIfNotEmpty {
        super::super::Chain(
            AppendTrailingCommaIfNotEmpty(self.0),
            self.1.append_trailing_comma_if_not_empty(),
        )
    }

    type ChainWithComma<Other: traits::EmptyOrCommaSeparatedElements> =
        ChainWithComma<I, ThisOther::ChainWithComma<Other>>;

    fn chain_with_comma<Other: traits::EmptyOrCommaSeparatedElements>(
        self,
        other: Other,
    ) -> Self::ChainWithComma<Other> {
        ChainWithComma(self.0, self.1.chain_with_comma(other))
    }
}

pub struct ChainWithCommaChunks<
    I: Iterator<Item: traits::Text>,
    Other: traits::EmptyOrCommaSeparatedElements,
>(ChainWithCommaChunksInner<MapIntoTextChunks<I>, Other>);
impl<I: Iterator<Item: traits::Text>, Other: traits::EmptyOrCommaSeparatedElements>
    ChainWithCommaChunks<I, Other>
{
    fn new(mut iter: I, other: Other) -> Self {
        Self(match iter.next() {
            Some(cur) => ChainWithCommaChunksInner::Both(
                Some((cur.into_text_chunks(), MapIntoTextChunks { iter })),
                other
                    .prepend_leading_comma_if_not_empty()
                    .into_text_chunks(),
            ),
            None => ChainWithCommaChunksInner::OnlyOther(other.into_text_chunks()),
        })
    }
}

impl<I: Iterator<Item: traits::Text>, Other: traits::EmptyOrCommaSeparatedElements> IterTextChunk
    for ChainWithCommaChunks<I, Other>
{
    type Chunk<'a>
        = ChainWithCommaChunkOf<'a, <I::Item as IntoTextChunks>::IntoTextChunks, Other>
    where
        Self: 'a;

    fn next_text_chunk(&mut self) -> Option<Self::Chunk<'_>> {
        self.0.next_text_chunk()
    }
    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        self.0.bytes_len_hint()
    }
}

enum ChainWithCommaChunksInner<
    I: Iterator<Item: IterTextChunk>,
    Other: traits::EmptyOrCommaSeparatedElements,
> {
    Both(
        Option<(I::Item, I)>,
        <Other::PrependLeadingCommaIfNotEmpty as IntoTextChunks>::IntoTextChunks,
    ),
    OnlyOther(Other::IntoTextChunks),
}

pub enum ChainWithCommaChunk<C, OtherChunk, OtherPrependLeadingCommaChunk> {
    Comma,
    Chunk(C),
    OtherChunk(OtherChunk),
    OtherPrependLeadingCommaChunk(OtherPrependLeadingCommaChunk),
}

impl<C: AsRef<[u8]>, OtherChunk: AsRef<[u8]>, OtherPrependLeadingCommaChunk: AsRef<[u8]>>
    AsRef<[u8]> for ChainWithCommaChunk<C, OtherChunk, OtherPrependLeadingCommaChunk>
{
    fn as_ref(&self) -> &[u8] {
        match self {
            ChainWithCommaChunk::Comma => b",",
            ChainWithCommaChunk::Chunk(c) => c.as_ref(),
            ChainWithCommaChunk::OtherChunk(c) => c.as_ref(),
            ChainWithCommaChunk::OtherPrependLeadingCommaChunk(c) => c.as_ref(),
        }
    }
}

type ChainWithCommaChunkOf<'a,Item,Other>=
ChainWithCommaChunk<
        <Item as IterTextChunk>::Chunk<'a>,
        <<Other as IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>,
        <<<Other as traits::EmptyOrCommaSeparatedElements>::PrependLeadingCommaIfNotEmpty as IntoTextChunks>::IntoTextChunks as IterTextChunk>::Chunk<'a>,

    >;

impl<I: Iterator<Item: IterTextChunk>, Other: traits::EmptyOrCommaSeparatedElements>
    ChainWithCommaChunksInner<I, Other>
{
    fn next_text_chunk(&mut self) -> Option<ChainWithCommaChunkOf<'_, I::Item, Other>> {
        match self {
            ChainWithCommaChunksInner::Both(cur_and_iter, other) => {
                enum Out {
                    AssignNoneToCurAndIter,
                    AssertCurAndIterIsNone,
                }

                match polonius::<_, Out, ForLt![Option<ChainWithCommaChunkOf<'_, I::Item, Other>>]>(
                    cur_and_iter,
                    |cur_and_iter| match cur_and_iter {
                        Some((cur, iter)) => {
                            match polonius::<
                                _,
                                (),
                                ForLt![Option<ChainWithCommaChunkOf<'_, I::Item, Other>>],
                            >(cur, |cur| {
                                match cur.next_text_chunk() {
                                    Some(v) => PoloniusResult::Borrowing(Some(
                                        ChainWithCommaChunk::Chunk(v),
                                    )),
                                    None => PoloniusResult::Owned(()),
                                }
                            }) {
                                PoloniusResult::Borrowing(v) => {
                                    return PoloniusResult::Borrowing(v);
                                }
                                PoloniusResult::Owned {
                                    value: (),
                                    input_borrow: cur,
                                } => match iter.next() {
                                    Some(next_item) => {
                                        *cur = next_item;
                                        return PoloniusResult::Borrowing(Some(
                                            ChainWithCommaChunk::Comma,
                                        ));
                                    }
                                    None => PoloniusResult::Owned(Out::AssignNoneToCurAndIter),
                                },
                            }
                        }
                        None => PoloniusResult::Owned(Out::AssertCurAndIterIsNone),
                    },
                ) {
                    PoloniusResult::Borrowing(v) => v,
                    PoloniusResult::Owned {
                        value,
                        input_borrow: cur_and_iter,
                    } => {
                        match value {
                            Out::AssignNoneToCurAndIter => {
                                *cur_and_iter = None;
                            }
                            Out::AssertCurAndIterIsNone => {
                                debug_assert!(cur_and_iter.is_none());
                            }
                        }
                        other
                            .next_text_chunk()
                            .map(ChainWithCommaChunk::OtherPrependLeadingCommaChunk)
                    }
                }
            }
            ChainWithCommaChunksInner::OnlyOther(other) => {
                other.next_text_chunk().map(ChainWithCommaChunk::OtherChunk)
            }
        }
    }

    fn bytes_len_hint(&self) -> (usize, Option<usize>) {
        match self {
            ChainWithCommaChunksInner::Both(cur_and_iter, other) => match cur_and_iter {
                Some((cur, iter)) => {
                    (SizeHint(cur.bytes_len_hint()) + (SizeHint(iter.size_hint()) * 2)).0
                }
                None => other.bytes_len_hint(),
            },
            ChainWithCommaChunksInner::OnlyOther(other) => other.bytes_len_hint(),
        }
    }
}
