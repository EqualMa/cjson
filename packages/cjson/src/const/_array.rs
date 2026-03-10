#![cfg(todo)]

use core::marker::PhantomData;

use crate::ser::{texts, traits};

use self::bytes_or_len::BytesOrLen;

mod bytes_or_len;

pub struct Chain<A, B>(A, B);

pub struct ConstChunk<T: ?Sized + HasConstChunk>(PhantomData<T>);

pub trait HasConstChunk {
    const CHUNK: &'static str;
}

pub struct TextChunks<State, Runtime, const CAP: usize>(State, Runtime, BytesOrLen<CAP>);

type EmptyArray = TextChunks<state::EmptyArray, Nothing, 2>;

pub struct Nothing;

pub struct ArrayWithValues<const LEN: usize>(BytesOrLen<LEN>);

impl<AfterLeftBracket, Runtime, const CAP: usize>
    TextChunks<state::LeftBracket<Nothing, AfterLeftBracket>, Runtime, CAP>
{
    #[cfg(todo)]
    pub const fn with_right_bracket(
        self,
    ) -> TextChunks<state::Array<AfterLeftBracket>, Runtime, CAP> {
        // let Self(state::LeftBracket, runtime_chunks, const_chunks) = self;

        todo!()
    }
}

impl<BeforeLeftBracket, AfterLeftBracket, Runtime, const CAP: usize>
    TextChunks<
        state::LeftBracket<
            LeftBracket<
                //
                BeforeOuterLeftBracket,
                AfterOuterRightBracket,
            >,
            AfterLeftBracket,
        >,
        Runtime,
        CAP,
    >
{
}

impl<BeforeLeftBracket, AfterLeftBracket, Runtime, const CAP: usize>
    TextChunks<state::LeftBracket<BeforeLeftBracket, AfterLeftBracket>, Runtime, CAP>
{
    pub const fn with_left_bracket(
        self,
    ) -> TextChunks<
        state::LeftBracket<state::LeftBracket<BeforeLeftBracket, AfterLeftBracket>, Nothing>,
        Runtime,
        CAP,
    > {
    }

    pub const fn push_value(
        mut self,
        val: texts::Value<&'static str>,
    ) -> TextChunks<state::LeftBracketWithValueCommaList, CAP> {
        self.1.push_str(val.inner());
        self.1.push_str(",");

        TextChunks(state::LeftBracketWithValueCommaList, self.1)
    }

    pub const fn end_with_right_bracket(self) -> EmptyArray {
        assert!(self.1.len() == CAP); // bytes should be just filled
        assert!(matches!(self.1.as_bytes(), b"[]"));

        EmptyArray
    }
}

impl<const LEN: usize> TextChunks<state::LeftBracketWithValueCommaList, LEN> {
    pub const fn push_value(mut self, val: texts::Value<&'static str>) -> Self {
        self.1.push_str(val.inner());
        self.1.push_str(",");

        self
    }

    pub const fn end_with_right_bracket(self) -> ArrayWithValues<LEN> {
        assert!(self.1.len() == LEN); // bytes should be just filled
        ArrayWithValues(self.1)
    }
}

pub struct State<T>(T);

macro_rules! State {
    ($Before:ty, '[', $After:ty) => {
        state::LeftBracketSeparated<$Before, $After>
    };
}

/// `[ state::ValueCommaList`
impl<Runtime, const CAP: usize> TextChunks<state::LeftBracketWithValueCommaList, Runtime, CAP> {
    pub const fn with_right_bracket(self) -> TextChunks<state::Array, Runtime, CAP> {
        todo!()
    }

    pub const fn with_json_value(self, json_value: texts::Value<&str>) -> Self {
        todo!()
    }
}

/// `BeforeOuterLB [ state::ValueCommaList [ state::ValueCommaList`
impl<Runtime, const CAP: usize, BeforeOuterLB>
    TextChunks<
        state::LeftBracketSeparated<
            //
            state::LeftBracketSeparated<BeforeOuterLB, state::ValueCommaList>,
            state::ValueCommaList,
        >,
        Runtime,
        CAP,
    >
{
    /// `+ ] = BeforeOuterLB [ state::ValueCommaList`
    pub const fn with_right_bracket(
        self,
    ) -> TextChunks<state::LeftBracketSeparated<BeforeOuterLB, state::ValueCommaList>, Runtime, CAP>
    {
        todo!()
    }

    /// `+ json_value = Self`
    pub const fn with_json_value(self, json_value: texts::Value<&str>) -> Self {
        todo!()
    }

    pub const fn with_runtime_json_value<T: ?Sized + HasConstChunk, const NEW_CAP: usize>(
        self,
        json_value: impl traits::Value,
    ) -> TextChunks<
        state::LeftBracketSeparated<
            //
            state::LeftBracketSeparated<BeforeOuterLB, state::ValueCommaList>,
            state::ValueCommaList,
        >,
        Chain<Runtime, ConstChunk<T>>,
        NEW_CAP,
    > {
        let Self(_, runtime_chunks, const_chunks) = self;

        assert!(*const_chunks.as_bytes() == *T::CHUNK.as_bytes());

        todo!()
    }
}

mod state {
    pub struct Nothing;
    pub struct ValueCommaList;
    pub type LeftBracketWithValueCommaList = LeftBracketSeparated<Nothing, ValueCommaList>;

    pub struct LeftBracketSeparated<Before, After>(
        //
        pub(crate) Before,
        pub(crate) After,
    );

    pub struct Array;

    pub struct EmptyArray;

    pub struct NonEmptyArray;
}
