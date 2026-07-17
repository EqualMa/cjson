use core::{marker::PhantomData, mem::transmute};

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::{
    ser::{
        ToJson, ToJsonArray, ToJsonString,
        texts::{self, Chain},
        traits::{
            self, Array, ConsumeTextChunk, EmptyOrCommaSeparatedElements, IntoTextChunks,
            JsonString, TryConsumeTextChunk,
        },
    },
    utils::impl_many,
};

pub use self::{
    constrained_types::{
        //
        ContentfulFirstChunkOfArrayAsArray,
        ContentfulFirstChunkOfArrayAsArrayVec,
        ContentfulFirstChunkOfArrayAsStr,
        //
        ContentfulFirstChunkOfObjectAsArray,
        ContentfulFirstChunkOfObjectAsArrayVec,
        ContentfulFirstChunkOfObjectAsStr,
        //
        ContentfulLastChunkOfArrayAsArray,
        ContentfulLastChunkOfArrayAsArrayVec,
        ContentfulLastChunkOfArrayAsStr,
        //
        ContentfulLastChunkOfObjectAsArray,
        ContentfulLastChunkOfObjectAsArrayVec,
        ContentfulLastChunkOfObjectAsStr,
        //
        FirstChunkOfJsonStringAsArray,
        FirstChunkOfJsonStringAsArrayVec,
        FirstChunkOfJsonStringAsStr,
        //
        IntermediateChunkAsArray,
        IntermediateChunkAsArrayVec,
        IntermediateChunkAsStr,
        //
        JsonStringAsArray,
        JsonStringAsArrayVec,
        JsonStringAsStr,
        //
        JsonStringFragmentAsArray,
        JsonStringFragmentAsArrayVec,
        JsonStringFragmentAsStr,
        //
        LastChunkOfJsonStringAsArray,
        LastChunkOfJsonStringAsArrayVec,
        LastChunkOfJsonStringAsStr,
        //
        NonEmptyArrayAsArray,
        NonEmptyArrayAsArrayVec,
        NonEmptyArrayAsStr,
        //
        NonEmptyObjectAsArray,
        NonEmptyObjectAsArrayVec,
        NonEmptyObjectAsStr,
    },
    stated_str::StatedChunkStr,
    stated_str_as_array::StatedChunkString,
    stated_str_as_array_vec::StatedChunkBuf,
};

pub mod states;

pub mod value;

pub mod array;
pub mod object;
pub mod string;

mod str_as_array;
mod str_as_array_vec;

mod stated_str;
mod stated_str_as_array;
mod stated_str_as_array_vec;

mod constrained_types;

pub struct ConstIntoJson<T>(pub T);

pub struct ConstIntoJsonValueString<T>(pub T);
pub struct ConstAsJsonValueStr<T>(pub T);

pub struct ConstIntoTextChunks<T: ?Sized>(pub T);
pub struct ConstIterTextChunk<T: ?Sized>(pub T);

pub struct ConstIntoJsonStringFragment<T>(pub T);

pub struct BooleanTextChunks(Option<bool>);

impl BooleanTextChunks {
    const fn next_text_chunk() {}
}

#[derive(Debug, RefCastCustom)]
#[repr(transparent)]
pub struct AsRefU8Slice<T>(pub T);

impl<T> AsRefU8Slice<T> {
    #[ref_cast_custom]
    pub(crate) const fn new_ref(s: &T) -> &Self;
}

pub trait HasConstJsonValue {
    const JSON_VALUE: texts::Value<&'static str>;
}

pub trait HasConstState {
    const STATE: State;
}

mod sealed {
    pub trait HasConstJsonArray {}
}

/// Asserts [`Self::JSON_VALUE`] starts with `[` and ends with `]`.
///
/// [`Self::JSON_VALUE`]: HasConstJsonValue::JSON_VALUE
pub trait HasConstJsonArray: HasConstJsonValue + sealed::HasConstJsonArray {}

pub struct ConstJsonValue<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> ConstJsonValue<T> {
    pub const DEFAULT: Self = Self(PhantomData);
    pub const fn new() -> Self {
        Self::DEFAULT
    }
}

impl<T: ?Sized> Copy for ConstJsonValue<T> {}
impl<T: ?Sized> Clone for ConstJsonValue<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Default for ConstJsonValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct JsonValueString<const LEN: usize>([u8; LEN]);

impl<T: ?Sized + HasConstJsonValue> ConstJsonValue<T> {
    pub const fn as_json_value_str(self) -> texts::Value<&'static str> {
        T::JSON_VALUE
    }
}

mod ser {
    use core::marker::PhantomData;

    use crate::{
        ser::{
            ToJson,
            iter_text_chunk::IterNonLending,
            texts::{self, ConstChunk, Empty},
            traits::{self, IntoTextChunks},
        },
        values::Either,
    };

    use super::{ConstJsonValue, HasConstJsonArray, HasConstJsonValue};

    pub struct Chunk<T: ?Sized + HasConstJsonValue>(PhantomData<T>);

    impl<T: ?Sized + HasConstJsonValue> AsRef<[u8]> for Chunk<T> {
        fn as_ref(&self) -> &[u8] {
            T::JSON_VALUE.inner().as_bytes()
        }
    }

    impl<T: ?Sized + HasConstJsonValue> ToJson for ConstJsonValue<T> {
        type ToJson<'a>
            = Self
        where
            Self: 'a;

        fn to_json(&self) -> Self::ToJson<'_> {
            *self
        }
    }

    impl<T: ?Sized + HasConstJsonValue> IntoTextChunks for ConstJsonValue<T> {
        type IntoTextChunks = IterNonLending<core::iter::Once<Chunk<T>>>;

        fn into_text_chunks(self) -> Self::IntoTextChunks {
            IterNonLending(core::iter::once(Chunk(PhantomData)))
        }

        crate::ser::traits::proxy_IntoTextChunks_write!(|self| -> texts::Value<&'static str> {
            T::JSON_VALUE
        });
    }

    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Text for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::sealed::Value for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonValue> traits::Value for ConstJsonValue<T> {}

    impl<T: ?Sized + HasConstJsonArray> traits::sealed::Array for ConstJsonValue<T> {}
    impl<T: ?Sized + HasConstJsonArray> traits::Array for ConstJsonValue<T> {
        type IntoCommaSeparatedElements =
            Either<texts::NonEmptyCommaSeparatedItems<ConstChunk<ConstNonEmptyItems<T>>>, Empty>;

        fn into_comma_separated_elements(self) -> Self::IntoCommaSeparatedElements {
            const {
                let items = array_items(T::JSON_VALUE.inner());
                if items.is_empty() {
                    Either::B(Empty)
                } else {
                    Either::A(texts::NonEmptyCommaSeparatedItems::new_without_validation(
                        ConstChunk::DEFAULT,
                    ))
                }
            }
        }
    }

    enum Never {}
    pub struct ConstNonEmptyItems<T: ?Sized + HasConstJsonArray>(Never, PhantomData<T>);

    const fn array_items(arr: &str) -> &str {
        let (lb, after_lb) = arr.split_at(1);
        assert!(matches!(lb.as_bytes(), b"["));

        let (items, rb) = after_lb.split_at(after_lb.len() - 1);
        assert!(matches!(rb.as_bytes(), b"]"));

        items
    }

    impl<T: ?Sized + HasConstJsonArray> crate::ser::iter_text_chunk::HasConstChunk
        for ConstNonEmptyItems<T>
    {
        const CHUNK: &'static str = {
            let items = array_items(T::JSON_VALUE.inner());

            assert!(!items.is_empty());
            items
        };
    }

    mod r#const {}
}

#[cfg(test)]
mod const_tests {
    use super::{ConstIntoJson, ConstIntoTextChunks, ConstIterTextChunk};

    const fn assert_ser_bool(v: bool) {
        let mut chunks = ConstIterTextChunk(
            ConstIntoTextChunks(ConstIntoJson(v).const_into_json()).const_into_text_chunks(),
        );

        let Some(chunk) = chunks.const_next_text_chunk() else {
            panic!()
        };
        let chunk = chunk.as_ref_u8_slice();

        if v {
            assert!(matches!(chunk, b"true"));
        } else {
            assert!(matches!(chunk, b"false"));
        }

        assert!(chunks.const_next_text_chunk().is_none());
    }

    const _: () = {
        assert_ser_bool(true);
        assert_ser_bool(false);
    };
}

pub(crate) mod array_string;
pub(crate) mod json_value_array_str;

mod state;

pub use self::state::{
    CompileTimeChunk, CompileTimeChunkIsJsonValue, HasConstCompileTimeChunk, State,
};

pub(crate) use self::state::assert_json_value;

pub struct StatedChunkStr2<'a, const PREV_STATE: u128, const NEXT_STATE: u128>(&'a str);

pub struct ChunkLen(usize);

impl ChunkLen {
    pub const DEFAULT: Self = Self(0);

    pub const fn len(self) -> usize {
        self.0
    }

    pub const fn left_bracket(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn right_bracket(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn left_brace(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn right_brace(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn comma(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn colon(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn double_quote(mut self) -> Self {
        self.0 += 1;
        self
    }

    pub const fn json_value(mut self, len: usize) -> Self {
        assert!(len > 0);
        self.0 += len;
        self
    }

    pub const fn json_value_generic_const(self, cap: usize) -> Self {
        assert!(
            cap > 0,
            "The capacity is 0 in json_value_generic_const!(_, capacity)"
        );
        self.json_value(cap)
    }

    pub const fn json_string_fragment(mut self, len: usize) -> Self {
        self.0 += len;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonValue<C: RuntimeChunk, V: ToJson>(pub C, pub V);

impl<C: RuntimeChunk, V: ToJson> ChunkConcatJsonValue<C, V> {
    const IMPL_NEXT_STATE: State = C::NEXT_STATE.json_value();
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonStringFragment<C: RuntimeChunk, V: ToJsonString>(pub C, pub V);

impl<C: RuntimeChunk, V: ToJsonString> ChunkConcatJsonStringFragment<C, V> {
    const IMPL_NEXT_STATE: State = C::NEXT_STATE.json_string_fragment();
}

#[derive(Debug, Clone, Copy)]
pub struct ChunkConcat<A: RuntimeChunk, B: RuntimeChunk>(pub A, pub B);

impl<A: RuntimeChunk, B: RuntimeChunk> ChunkConcat<A, B> {
    const ASSERT: () = {
        A::NEXT_STATE.assert_same(&B::PREV_STATE);
    };
}

impl<A: RuntimeChunk, B: RuntimeChunk> RuntimeChunk for ChunkConcat<A, B> {
    const PREV_STATE: State = {
        () = Self::ASSERT;
        A::PREV_STATE
    };
    const NEXT_STATE: State = {
        () = Self::ASSERT;
        B::NEXT_STATE
    };

    type ToIntoTextChunks<'a>
        = Chain<A::ToIntoTextChunks<'a>, B::ToIntoTextChunks<'a>>
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        const { () = Self::ASSERT }

        Chain(
            //
            self.0.to_into_text_chunks(),
            self.1.to_into_text_chunks(),
        )
    }

    fn runtime_chunk_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
        self.0.runtime_chunk_write_into(w);
        self.1.runtime_chunk_write_into(w)
    }
    fn runtime_chunk_try_write_into<W: ?Sized + TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        self.0.runtime_chunk_try_write_into(w)?;
        self.1.runtime_chunk_try_write_into(w)
    }
}

impl<A: RuntimeChunkStartingWithCompileTime, B: RuntimeChunk> RuntimeChunkStartingWithCompileTime
    for ChunkConcat<A, B>
{
    const PREV_STATE_REMOVE_GROUP_OPEN: State = {
        () = Self::ASSERT;
        A::PREV_STATE_REMOVE_GROUP_OPEN
    };

    type ChunksReadyToRemoveGroupOpen<'a>
        = Chain<A::ChunksReadyToRemoveGroupOpen<'a>, B::ToIntoTextChunks<'a>>
    where
        Self: 'a;

    fn to_text_chunks_ready_to_remove_group_open(&self) -> Self::ChunksReadyToRemoveGroupOpen<'_> {
        const { _ = Self::PREV_STATE_REMOVE_GROUP_OPEN }
        Chain(
            //
            self.0.to_text_chunks_ready_to_remove_group_open(),
            self.1.to_into_text_chunks(),
        )
    }
}

impl<A: RuntimeChunk, B: RuntimeChunkEndingWithCompileTime> RuntimeChunkEndingWithCompileTime
    for ChunkConcat<A, B>
{
    const NEXT_STATE_REMOVE_GROUP_CLOSE: State = {
        () = Self::ASSERT;
        B::NEXT_STATE_REMOVE_GROUP_CLOSE
    };

    type ChunksReadyToRemoveGroupClose<'a>
        = Chain<A::ToIntoTextChunks<'a>, B::ChunksReadyToRemoveGroupClose<'a>>
    where
        Self: 'a;

    fn to_text_chunks_ready_to_remove_group_close(
        &self,
    ) -> Self::ChunksReadyToRemoveGroupClose<'_> {
        const { _ = Self::NEXT_STATE_REMOVE_GROUP_CLOSE }
        Chain(
            //
            self.0.to_into_text_chunks(),
            self.1.to_text_chunks_ready_to_remove_group_close(),
        )
    }
}

impl<A: TextChunksReadyToRemoveGroupOpen, B: IntoTextChunks> TextChunksReadyToRemoveGroupOpen
    for Chain<A, B>
{
    type RemoveGroupOpen = Chain<A::RemoveGroupOpen, B>;

    fn remove_group_open(self) -> Self::RemoveGroupOpen {
        Chain(self.0.remove_group_open(), self.1)
    }
}

impl<A: IntoTextChunks, B: TextChunksReadyToRemoveGroupClose> TextChunksReadyToRemoveGroupClose
    for Chain<A, B>
{
    type RemoveGroupClose = Chain<A, B::RemoveGroupClose>;

    fn remove_group_close(self) -> Self::RemoveGroupClose {
        Chain(self.0, self.1.remove_group_close())
    }
}

impl<A: RuntimeChunkStartingWithCompileTime, B: RuntimeChunkEndingWithCompileTime>
    RuntimeChunkSurroundedWithCompileTime for ChunkConcat<A, B>
{
    type ChunksReadyToUngroup<'a>
        = Chain<A::ChunksReadyToRemoveGroupOpen<'a>, B::ChunksReadyToRemoveGroupClose<'a>>
    where
        Self: 'a;

    const UNGROUPED_STATES: (State, State) = {
        A::NEXT_STATE.assert_same(&B::PREV_STATE);
        (
            A::PREV_STATE_REMOVE_GROUP_OPEN,
            B::NEXT_STATE_REMOVE_GROUP_CLOSE,
        )
    };

    fn to_text_chunks_ready_to_ungroup(&self) -> Self::ChunksReadyToUngroup<'_> {
        const { _ = Self::UNGROUPED_STATES }
        Chain(
            //
            self.0.to_text_chunks_ready_to_remove_group_open(),
            self.1.to_text_chunks_ready_to_remove_group_close(),
        )
    }
}

impl<A: TextChunksReadyToRemoveGroupOpen, B: TextChunksReadyToRemoveGroupClose>
    TextChunksReadyToUngroup for Chain<A, B>
{
    type Ungroup = Chain<A::RemoveGroupOpen, B::RemoveGroupClose>;

    fn ungroup(self) -> Self::Ungroup {
        Chain(self.0.remove_group_open(), self.1.remove_group_close())
    }
}

// TODO: sealed
pub trait RuntimeChunk {
    const PREV_STATE: State;
    const NEXT_STATE: State;

    type ToIntoTextChunks<'a>: IntoTextChunks
    where
        Self: 'a;
    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_>;

    fn runtime_chunk_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W);
    fn runtime_chunk_try_write_into<W: ?Sized + TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err>;
}

#[cfg(todo)]
impl<'this, C: ?Sized + RuntimeChunk> RuntimeChunk for &'this C {
    const PREV_STATE: State = C::PREV_STATE;
    const NEXT_STATE: State = C::NEXT_STATE;

    type ToIntoTextChunks<'a>
        = C::ToIntoTextChunks<'this>
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        C::to_into_text_chunks(self)
    }
}

pub trait TextChunksReadyToRemoveGroupOpen: IntoTextChunks {
    type RemoveGroupOpen: IntoTextChunks;
    fn remove_group_open(self) -> Self::RemoveGroupOpen;
}

pub trait TextChunksReadyToRemoveGroupClose: IntoTextChunks {
    type RemoveGroupClose: IntoTextChunks;
    fn remove_group_close(self) -> Self::RemoveGroupClose;
}

// TODO: sealed
pub trait RuntimeChunkStartingWithCompileTime: RuntimeChunk + Sized {
    const PREV_STATE_REMOVE_GROUP_OPEN: State;

    type ChunksReadyToRemoveGroupOpen<'a>: TextChunksReadyToRemoveGroupOpen
    where
        Self: 'a;
    fn to_text_chunks_ready_to_remove_group_open(&self) -> Self::ChunksReadyToRemoveGroupOpen<'_>;
}

// TODO: sealed
pub trait RuntimeChunkEndingWithCompileTime: RuntimeChunk + Sized {
    const NEXT_STATE_REMOVE_GROUP_CLOSE: State;

    type ChunksReadyToRemoveGroupClose<'a>: TextChunksReadyToRemoveGroupClose
    where
        Self: 'a;
    fn to_text_chunks_ready_to_remove_group_close(&self)
    -> Self::ChunksReadyToRemoveGroupClose<'_>;
}

pub trait TextChunksReadyToUngroup: IntoTextChunks {
    type Ungroup: IntoTextChunks;
    fn ungroup(self) -> Self::Ungroup;
}

// TODO: sealed
pub trait RuntimeChunkSurroundedWithCompileTime: RuntimeChunk {
    const UNGROUPED_STATES: (State, State);
    type ChunksReadyToUngroup<'a>: TextChunksReadyToUngroup
    where
        Self: 'a;
    fn to_text_chunks_ready_to_ungroup(&self) -> Self::ChunksReadyToUngroup<'_>;
}

impl<T: ?Sized + HasConstCompileTimeChunk> RuntimeChunk for CompileTimeChunk<T> {
    const PREV_STATE: State = T::CHUNK.into_prev_state();
    const NEXT_STATE: State = T::CHUNK.into_next_state();

    type ToIntoTextChunks<'a>
        = Self
    where
        Self: 'a;

    fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
        Self::DEFAULT
    }

    fn runtime_chunk_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
        w.consume_text_chunk(const { T::CHUNK.into_inner() });
    }

    fn runtime_chunk_try_write_into<W: ?Sized + TryConsumeTextChunk>(
        self,
        w: &mut W,
    ) -> Result<(), W::Err> {
        w.try_consume_text_chunk(const { T::CHUNK.into_inner() })
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> TextChunksReadyToRemoveGroupOpen
    for CompileTimeChunk<T>
{
    type RemoveGroupOpen = CompileTimeChunk<ConstRemoveGroupOpen<T>>;

    fn remove_group_open(self) -> Self::RemoveGroupOpen {
        CompileTimeChunk::DEFAULT
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> TextChunksReadyToRemoveGroupClose
    for CompileTimeChunk<T>
{
    type RemoveGroupClose = CompileTimeChunk<ConstRemoveGroupClose<T>>;

    fn remove_group_close(self) -> Self::RemoveGroupClose {
        CompileTimeChunk::DEFAULT
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> TextChunksReadyToUngroup for CompileTimeChunk<T> {
    type Ungroup = CompileTimeChunk<ConstRemoveSurroundingGroup<T>>;

    fn ungroup(self) -> Self::Ungroup {
        CompileTimeChunk::DEFAULT
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> RuntimeChunkStartingWithCompileTime
    for CompileTimeChunk<T>
{
    const PREV_STATE_REMOVE_GROUP_OPEN: State = {
        let chunk = <ConstRemoveGroupOpen<T> as HasConstCompileTimeChunk>::CHUNK;

        chunk.next_state().assert_same(&Self::NEXT_STATE);

        chunk.into_prev_state()
    };

    type ChunksReadyToRemoveGroupOpen<'a>
        = Self
    where
        Self: 'a;

    fn to_text_chunks_ready_to_remove_group_open(&self) -> Self::ChunksReadyToRemoveGroupOpen<'_> {
        const {
            _ = Self::PREV_STATE_REMOVE_GROUP_OPEN;
            Self::DEFAULT
        }
    }
}

impl<T: ?Sized + HasConstCompileTimeChunk> RuntimeChunkEndingWithCompileTime
    for CompileTimeChunk<T>
{
    const NEXT_STATE_REMOVE_GROUP_CLOSE: State = {
        let chunk = <ConstRemoveGroupClose<T> as HasConstCompileTimeChunk>::CHUNK;

        chunk.prev_state().assert_same(&Self::PREV_STATE);

        chunk.into_next_state()
    };

    type ChunksReadyToRemoveGroupClose<'a>
        = Self
    where
        Self: 'a;

    fn to_text_chunks_ready_to_remove_group_close(
        &self,
    ) -> Self::ChunksReadyToRemoveGroupClose<'_> {
        const {
            _ = Self::NEXT_STATE_REMOVE_GROUP_CLOSE;
            Self::DEFAULT
        }
    }
}

// TODO: is this needed?
impl<T: ?Sized + HasConstCompileTimeChunk> RuntimeChunkSurroundedWithCompileTime
    for CompileTimeChunk<T>
{
    type ChunksReadyToUngroup<'a>
        = Self
    where
        Self: 'a;

    const UNGROUPED_STATES: (State, State) = {
        let chunk = <ConstRemoveSurroundingGroup<T> as HasConstCompileTimeChunk>::CHUNK;

        (chunk.prev_state().copied(), chunk.into_next_state())
    };

    fn to_text_chunks_ready_to_ungroup(&self) -> Self::ChunksReadyToUngroup<'_> {
        const {
            _ = Self::UNGROUPED_STATES;
            CompileTimeChunk::DEFAULT
        }
    }
}

enum Never {}
pub struct ConstRemoveSurroundingGroup<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);
pub struct ConstRemoveGroupOpen<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);
pub struct ConstRemoveGroupClose<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

impl<T: ?Sized + HasConstCompileTimeChunk> HasConstCompileTimeChunk
    for ConstRemoveSurroundingGroup<T>
{
    const CHUNK: StatedChunkStr<'static> = T::CHUNK.remove_surrounding_group();
}
impl<T: ?Sized + HasConstCompileTimeChunk> HasConstCompileTimeChunk for ConstRemoveGroupOpen<T> {
    const CHUNK: StatedChunkStr<'static> = T::CHUNK.remove_group_open();
}
impl<T: ?Sized + HasConstCompileTimeChunk> HasConstCompileTimeChunk for ConstRemoveGroupClose<T> {
    const CHUNK: StatedChunkStr<'static> = T::CHUNK.remove_group_close();
}

/// json_items_after_item
///
/// ```ignore
/// [v1, ..items,]    -> [v1 $(,$item)*     ]
/// [v1, ..items, v2] -> [v1 $(,$item)* , v2]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonItemsAfterItem<C: RuntimeChunk, V: ToJsonArray>(pub C, pub V);

type JsonItemsAfterItem<T> =
    <JsonItemsBetweenBrackets<T> as traits::EmptyOrCommaSeparatedElements>::PrependLeadingCommaIfNotEmpty;

/// json_items_after_array_start_before_item
///
/// ```ignore
/// [..items, v]      -> [   $($item,)*  v ]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonItemsAfterArrayStartBeforeItem<C: RuntimeChunk, V: ToJsonArray>(
    pub C,
    pub V,
);

type JsonItemsAfterArrayStartBeforeItem<T> =
    <JsonItemsBetweenBrackets<T> as traits::EmptyOrCommaSeparatedElements>::AppendTrailingCommaIfNotEmpty;

/// json_items_between_brackets
///
/// ```ignore
/// [..items]         -> [   $($item),*     ]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ChunkConcatJsonItemsBetweenBrackets<C: RuntimeChunk, V: ToJsonArray>(pub C, pub V);

type JsonItemsBetweenBrackets<T> = <T as traits::Array>::IntoCommaSeparatedElements;

impl<C: RuntimeChunk, V: ToJsonArray> ChunkConcatJsonItemsAfterItem<C, V> {
    const IMPL_NEXT_STATE: State = C::NEXT_STATE.json_items_after_item();
}

impl<C: RuntimeChunk, V: ToJsonArray> ChunkConcatJsonItemsAfterArrayStartBeforeItem<C, V> {
    const IMPL_NEXT_STATE: State = C::NEXT_STATE.json_items_after_array_start_before_item();
}

impl<C: RuntimeChunk, V: ToJsonArray> ChunkConcatJsonItemsBetweenBrackets<C, V> {
    const IMPL_NEXT_STATE: State = C::NEXT_STATE.json_items_between_brackets();
}

impl_many!({
    {
        {
            use ChunkConcatJsonValue as CR;
            use ToJson as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> = <V as ToJson>::ToJson<'a>;
            fn runtime_chunk_to_text_chunk<V: ToJson>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json(v)
            }
        }
        {
            use ChunkConcatJsonStringFragment as CR;
            use ToJsonString as ToTrait;
            type FragmentsOf<S> = <S as traits::JsonString>::IntoJsonStringFragments;
            type RuntimeChunkToTextChunk<'a, V> =
                FragmentsOf<<V as ToJsonString>::ToJsonString<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonString>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_string(v).into_json_string_fragments()
            }
        }
        {
            use ChunkConcatJsonItemsAfterItem as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsAfterItem<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v)
                    .into_comma_separated_elements()
                    .prepend_leading_comma_if_not_empty()
            }
        }
        {
            use ChunkConcatJsonItemsAfterArrayStartBeforeItem as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsAfterArrayStartBeforeItem<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v)
                    .into_comma_separated_elements()
                    .append_trailing_comma_if_not_empty()
            }
        }
        {
            use ChunkConcatJsonItemsBetweenBrackets as CR;
            use ToJsonArray as ToTrait;
            type RuntimeChunkToTextChunk<'a, V> =
                JsonItemsBetweenBrackets<<V as ToJsonArray>::ToJsonArray<'a>>;
            fn runtime_chunk_to_text_chunk<V: ToJsonArray>(v: &V) -> RuntimeChunkToTextChunk<V> {
                V::to_json_array(v).into_comma_separated_elements()
            }
        }
    }

    impl<C: RuntimeChunkStartingWithCompileTime, V: ToTrait> RuntimeChunk for CR<C, V> {
        const PREV_STATE: State = C::PREV_STATE;
        const NEXT_STATE: State = Self::IMPL_NEXT_STATE;

        type ToIntoTextChunks<'a>
            = Chain<
            //
            C::ToIntoTextChunks<'a>,
            RuntimeChunkToTextChunk<'a, V>,
        >
        where
            Self: 'a;
        fn to_into_text_chunks(&self) -> Self::ToIntoTextChunks<'_> {
            const {
                _ = Self::PREV_STATE;
                _ = Self::NEXT_STATE;
            }
            Chain(
                self.0.to_into_text_chunks(),
                runtime_chunk_to_text_chunk(&self.1),
            )
        }

        fn runtime_chunk_write_into<W: ?Sized + ConsumeTextChunk>(self, w: &mut W) {
            self.0.runtime_chunk_write_into(w);
            runtime_chunk_to_text_chunk(&self.1).write_into(w)
        }
        fn runtime_chunk_try_write_into<W: ?Sized + TryConsumeTextChunk>(
            self,
            w: &mut W,
        ) -> Result<(), W::Err> {
            self.0.runtime_chunk_try_write_into(w)?;
            runtime_chunk_to_text_chunk(&self.1).try_write_into(w)
        }
    }

    impl<C: RuntimeChunkStartingWithCompileTime, V: ToTrait> RuntimeChunkStartingWithCompileTime
        for CR<C, V>
    {
        type ChunksReadyToRemoveGroupOpen<'a>
            = Chain<
            //
            C::ChunksReadyToRemoveGroupOpen<'a>,
            RuntimeChunkToTextChunk<'a, V>,
        >
        where
            Self: 'a;

        const PREV_STATE_REMOVE_GROUP_OPEN: State = C::PREV_STATE_REMOVE_GROUP_OPEN;

        fn to_text_chunks_ready_to_remove_group_open(
            &self,
        ) -> Self::ChunksReadyToRemoveGroupOpen<'_> {
            const {
                _ = Self::PREV_STATE_REMOVE_GROUP_OPEN;
                _ = Self::NEXT_STATE;
            }
            Chain(
                self.0.to_text_chunks_ready_to_remove_group_open(),
                runtime_chunk_to_text_chunk(&self.1),
            )
        }
    }
});
