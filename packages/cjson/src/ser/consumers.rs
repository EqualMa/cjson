use core::marker::PhantomData;

use crate::{
    r#const::{HasConstCompileTimeChunk, State},
    ser::{
        consumers::open_close::{MakeChunks, OpenClose},
        traits::ConsumeTextChunk,
    },
};

pub mod json_kinds {
    use core::convert::Infallible;

    pub struct AnyValue;
    pub struct JsonString;
    pub struct Array;
    pub struct Object;

    pub trait YesOrNo {}

    impl YesOrNo for () {}
    impl YesOrNo for Infallible {}

    pub trait JsonKind: Sized {
        fn into_kind_of_any_value(self) -> AnyValue {
            AnyValue
        }

        type Union<Other: JsonKind>;
        fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other>;

        type UnionString;
        fn union_string(self, other: JsonString) -> Self::UnionString;

        type UnionArray;
        fn union_array(self, other: Array) -> Self::UnionArray;

        type UnionObject;
        fn union_object(self, other: Object) -> Self::UnionObject;

        type Contains<Other: JsonKind>: YesOrNo;

        type StringContainsSelf: YesOrNo;
        type ArrayContainsSelf: YesOrNo;
        type ObjectContainsSelf: YesOrNo;
    }

    impl JsonKind for AnyValue {
        type Union<Other: JsonKind> = Self;

        fn union<Other: JsonKind>(self, _: Other) -> Self::Union<Other> {
            self
        }

        type UnionString = Self;

        fn union_string(self, _: JsonString) -> Self::UnionString {
            self
        }

        type UnionArray = Self;

        fn union_array(self, _: Array) -> Self::UnionArray {
            self
        }

        type UnionObject = Self;

        fn union_object(self, _: Object) -> Self::UnionObject {
            self
        }

        type Contains<Other: JsonKind> = ();

        type StringContainsSelf = Infallible;
        type ArrayContainsSelf = Infallible;
        type ObjectContainsSelf = Infallible;
    }

    impl JsonKind for JsonString {
        type Union<Other: JsonKind> = Other::UnionString;

        fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
            other.union_string(self)
        }

        type UnionString = Self;

        fn union_string(self, _: JsonString) -> Self::UnionString {
            self
        }

        type UnionArray = AnyValue;

        fn union_array(self, Array: Array) -> Self::UnionArray {
            AnyValue
        }

        type UnionObject = AnyValue;

        fn union_object(self, Object: Object) -> Self::UnionObject {
            AnyValue
        }

        type Contains<Other: JsonKind> = Other::StringContainsSelf;

        type StringContainsSelf = ();
        type ArrayContainsSelf = Infallible;
        type ObjectContainsSelf = Infallible;
    }

    impl JsonKind for Array {
        type Union<Other: JsonKind> = Other::UnionArray;

        fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
            other.union_array(self)
        }

        type UnionString = AnyValue;

        fn union_string(self, JsonString: JsonString) -> Self::UnionString {
            AnyValue
        }

        type UnionArray = Array;

        fn union_array(self, Array: Array) -> Self::UnionArray {
            Array
        }

        type UnionObject = AnyValue;

        fn union_object(self, Object: Object) -> Self::UnionObject {
            AnyValue
        }

        type Contains<Other: JsonKind> = Other::ArrayContainsSelf;

        type StringContainsSelf = Infallible;
        type ArrayContainsSelf = ();
        type ObjectContainsSelf = Infallible;
    }

    impl JsonKind for Object {
        type Union<Other: JsonKind> = Other::UnionObject;

        fn union<Other: JsonKind>(self, other: Other) -> Self::Union<Other> {
            other.union_object(self)
        }

        type UnionString = AnyValue;

        fn union_string(self, JsonString: JsonString) -> Self::UnionString {
            AnyValue
        }

        type UnionArray = AnyValue;

        fn union_array(self, Array: Array) -> Self::UnionArray {
            AnyValue
        }

        type UnionObject = Self;

        fn union_object(self, Object: Object) -> Self::UnionObject {
            self
        }

        type Contains<Other: JsonKind> = Other::ObjectContainsSelf;

        type StringContainsSelf = Infallible;
        type ArrayContainsSelf = Infallible;
        type ObjectContainsSelf = ();
    }
}

use json_kinds::JsonKind;

pub trait IntoJson {
    type JsonKind: JsonKind;
    fn json_provide_into<W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>>(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W>;
}

pub struct Consumed<K: JsonKind, W: ?Sized>(K, PhantomData<W>);

pub struct ConsumerExpectingJsonText<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumerExpectingJsonText<W> {
    //
}

pub struct EmptyArray;
pub struct EmptyObject;

impl IntoJson for EmptyArray {
    type JsonKind = json_kinds::Array;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_empty_array(())
    }
}

impl IntoJson for EmptyObject {
    type JsonKind = json_kinds::Object;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_empty_object(())
    }
}

pub trait ConsumeJson {
    type ConsumeJsonKind: JsonKind;
    fn consume_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self>;

    type ConsumeChunksOfNonEmptyArray: ConsumeJsonChunks<json_kinds::Array>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray;

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self>;
}

pub trait ConsumeJsonChunks<K: JsonKind> {
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk>: ConsumeJsonChunks<K>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T>;

    fn end(self) -> Consumed<K, Self>;
}

pub struct ConsumeJsonText<W: ConsumeTextChunk>(pub W);
pub struct ConsumeChunksOfJsonArray<W: ConsumeTextChunk, S: ?Sized + HasConstState>(
    W,
    PhantomData<S>,
);

pub trait HasConstState {
    const STATE: State;
}

mod states {
    use core::marker::PhantomData;

    use crate::r#const::{HasConstCompileTimeChunk, State};

    use super::HasConstState;

    pub enum Init {}

    impl HasConstState for Init {
        const STATE: State = State::INIT;
    }

    enum Never {}
    pub struct NextStateOf<T: ?Sized + HasConstCompileTimeChunk>(Never, PhantomData<T>);

    impl<T: ?Sized + HasConstCompileTimeChunk> HasConstState for NextStateOf<T> {
        const STATE: State = T::CHUNK.next_state();
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeJsonText<W> {
    type ConsumeJsonKind = json_kinds::AnyValue;
    fn consume_empty_array(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk("[]");
        Consumed(json_kinds::Array, PhantomData)
    }

    type ConsumeChunksOfNonEmptyArray = ConsumeChunksOfJsonArray<W, states::Init>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    fn consume_empty_object(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk("{}");
        Consumed(json_kinds::Object, PhantomData)
    }
}

impl<W: ConsumeTextChunk, S: ?Sized + HasConstState> ConsumeJsonChunks<json_kinds::Array>
    for ConsumeChunksOfJsonArray<W, S>
{
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
        ConsumeChunksOfJsonArray<W, states::NextStateOf<T>>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        const {
            S::STATE.assert_same(T::CHUNK.prev_state());

            if S::STATE.is_init() {
                T::CHUNK
                    .remove_group_open()
                    .prev_state()
                    .assert_same(State::INIT_AFTER_ARRAY_START)
            }
        }
        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    fn end(self) -> Consumed<json_kinds::Array, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed(json_kinds::Array, PhantomData)
    }
}

mod open_close;

pub struct ConsumeArrayItems<W: ConsumeTextChunk>(pub W);
pub struct ConsumeChunksOfNonEmptyArray<
    W: ConsumeTextChunk,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<S>);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayItems<W> {
    type ConsumeJsonKind = json_kinds::Array;
    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        Consumed(json_kinds::Array, PhantomData)
    }
    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        match yes {}
    }
}

impl<W: ConsumeTextChunk, S: ?Sized + HasConstState, const OPEN_CLOSE: u8>
    ConsumeJsonChunks<json_kinds::Array> for ConsumeChunksOfNonEmptyArray<W, S, OPEN_CLOSE>
{
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
        ConsumeChunksOfJsonArray<W, states::NextStateOf<T>>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        mut self,
    ) -> Self::ConsumeConstChunk<T> {
        const {
            S::STATE.assert_same(T::CHUNK.prev_state());

            if T::CHUNK.prev_state().is_init() {
                T::CHUNK
                    .remove_group_open()
                    .prev_state()
                    .assert_is_top_level_after_array_start();
            }

            if T::CHUNK.next_state().is_eof() {
                T::CHUNK
                    .remove_group_close()
                    .next_state()
                    .assert_is_before_top_level_right_bracket();
            }
        }

        if const { <T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.prepend_comma } {
            self.0.consume_text_chunk(",");
        }

        if const { !(<T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.chunk.is_empty()) } {
            self.0
                .consume_text_chunk(<T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.chunk);
        }

        if const { <T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.append_comma } {
            self.0.consume_text_chunk(",");
        }

        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    fn end(self) -> Consumed<json_kinds::Array, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed(json_kinds::Array, PhantomData)
    }
}

pub struct ConsumeArrayItemsPrependCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayItemsPrependCommaIfNotEmpty<W> {
    type ConsumeJsonKind = json_kinds::Array;
    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        Consumed(json_kinds::Array, PhantomData)
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::PREPEND_COMMA.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        match yes {}
    }
}
