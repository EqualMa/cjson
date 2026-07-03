use core::marker::PhantomData;

use crate::{
    r#const::{HasConstCompileTimeChunk, State},
    ser::{
        consumers::{
            consume_chained_full::ConsumeChainedStringsFull,
            open_close::{MakeChunks, OpenClose},
            runtime_chunks::RuntimeChunks,
        },
        traits::{ConsumeTextChunk, IntoTextChunks as _},
    },
    utils::impl_many,
};

use super::IntoJson;

use self::{
    consume_chained_full::ConsumeChainedArraysFull,
    consume_open_content::ConsumeArrayOpenItemsIfNotEmpty, json_kinds::JsonKind,
    never_consume::NeverConsume,
};

pub use self::consumed::Consumed;

macro_rules! not_string {
    () => {
        fn consume_empty_string(
            self,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Consumed<json_kinds::JsonString, Self> {
            match yes {}
        }

        fn consume_str(
            self,
            _: &str,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Consumed<json_kinds::JsonString, Self> {
            match yes {}
        }

        type ConsumeChainedStrings =
            crate::ser::consumers::never_consume::NeverConsumeChained<Self>;
        fn start_to_consume_chained_strings(
            self,
            yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
        ) -> Self::ConsumeChainedStrings {
            match yes {}
        }
    };
}

macro_rules! not_array {
    () => {
        fn consume_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Array,
            >,
        ) -> Consumed<json_kinds::Array, Self> {
            match yes {}
        }

        type ConsumeChunksOfNonEmptyArray = crate::ser::consumers::never_consume::NeverConsume;

        fn start_to_consume_chunks_of_non_empty_array(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Array,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            match yes {}
        }

        type ConsumeChainedArrays = crate::ser::consumers::never_consume::NeverConsumeChained<Self>;
        fn start_to_consume_chained_arrays(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Array,
            >,
        ) -> Self::ConsumeChainedArrays {
            match yes {}
        }
    };
}

macro_rules! not_object {
    () => {
        fn consume_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Object,
            >,
        ) -> Consumed<json_kinds::Object, Self> {
            match yes {}
        }

        type ConsumeChunksOfNonEmptyObject = crate::ser::consumers::never_consume::NeverConsume;
        fn start_to_consume_chunks_of_non_empty_object(
            self,
            yes: <Self::ConsumeJsonKind as crate::ser::json_kinds::JsonKind>::Contains<
                json_kinds::Object,
            >,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            match yes {}
        }
    };
}

pub mod json_kinds;
pub mod runtime_chunks;

mod consume_chained_content;
mod consume_chained_full;
mod consume_content;
mod consume_content_and_record;
mod consume_content_close;
mod consume_open_content;
mod consumed;
mod never_consume;

pub trait ConsumeJson {
    type ConsumeJsonKind: JsonKind;

    fn consume_empty_string(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self>;

    fn consume_str(
        self,
        s: &str,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self>;

    type ConsumeChainedStrings: ConsumeChainedStrings<InitialConsumer = Self>;
    fn start_to_consume_chained_strings(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings;

    fn consume_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self>;

    type ConsumeChunksOfNonEmptyArray: ConsumeJsonChunks<json_kinds::Array>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray;

    type ConsumeChainedArrays: ConsumeChainedArrays<InitialConsumer = Self>;

    fn start_to_consume_chained_arrays(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays;

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self>;

    type ConsumeChunksOfNonEmptyObject: ConsumeJsonChunks<json_kinds::Object>;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject;
}

pub trait ConsumeChainedStrings {
    fn extend(&mut self, s: impl IntoJson<JsonKind = json_kinds::JsonString>);

    type InitialConsumer: ?Sized;
    fn end_with(
        self,
        s: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer>;
}

pub trait ConsumeChainedArrays {
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>);

    type InitialConsumer: ?Sized;
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer>;
}

pub trait ConsumeJsonChunks<K: JsonKind> {
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk>: ConsumeJsonChunks<K>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T>;

    type ConsumeRuntimeChunk<C: RuntimeChunks>: ConsumeJsonChunks<K>;
    fn consume_runtime_chunk<C: RuntimeChunks>(self, chunk: C) -> Self::ConsumeRuntimeChunk<C>;

    fn end(self) -> Consumed<K, Self>;
}

pub struct ConsumeJsonText<W: ConsumeTextChunk>(pub W);

// TODO: remove
pub struct ConsumeChunksOfJsonArray<W: ConsumeTextChunk, S: ?Sized + HasConstState>(
    W,
    PhantomData<S>,
);
pub struct ConsumeChunksOfJsonObject<W: ConsumeTextChunk, S: ?Sized + HasConstState>(
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

    fn consume_empty_string(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk("\"\"");
        Consumed::ASSERT_STRING
    }

    fn consume_str(
        mut self,
        s: &str,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        if s.is_empty() {
            self.consume_empty_string(())
        } else {
            self.0.consume_text_chunk("\"");
            super::texts::StrToJsonStringFragment(s).write_into(&mut self.0);
            self.0.consume_text_chunk("\"");
            Consumed::ASSERT_STRING
        }
    }

    type ConsumeChainedStrings = ConsumeChainedStringsFull<W>;
    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        ConsumeChainedStringsFull::new(self.0)
    }

    fn consume_empty_array(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk("[]");
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    type ConsumeChainedArrays = ConsumeChainedArraysFull<W>;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        ConsumeChainedArraysFull::new(self.0)
    }

    fn consume_empty_object(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk("{}");
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject =
        ConsumeChunksOfNonEmptyObject<W, states::Init, { OpenClose::BOTH_GROUP.as_u8() }>;

    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
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

    type ConsumeRuntimeChunk<C: RuntimeChunks> = ConsumeChunksOfJsonArray<W, C::NextState<S>>;
    fn consume_runtime_chunk<C: RuntimeChunks>(mut self, chunk: C) -> Self::ConsumeRuntimeChunk<C> {
        const { _ = C::NextState::<S>::STATE }
        chunk.runtime_chunks_write_into(&mut self.0);
        ConsumeChunksOfJsonArray(self.0, PhantomData)
    }

    fn end(self) -> Consumed<json_kinds::Array, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed::ASSERT_ARRAY
    }
}

impl<W: ConsumeTextChunk, S: ?Sized + HasConstState> ConsumeJsonChunks<json_kinds::Object>
    for ConsumeChunksOfJsonObject<W, S>
{
    type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
        ConsumeChunksOfJsonObject<W, states::NextStateOf<T>>;
    fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
        self,
    ) -> Self::ConsumeConstChunk<T> {
        const {
            S::STATE.assert_same(T::CHUNK.prev_state());

            if S::STATE.is_init() {
                T::CHUNK
                    .remove_group_open()
                    .prev_state()
                    .assert_same(State::INIT_AFTER_OBJECT_START)
            }
        }
        ConsumeChunksOfJsonObject(self.0, PhantomData)
    }

    type ConsumeRuntimeChunk<C: RuntimeChunks> = ConsumeChunksOfJsonObject<W, C::NextState<S>>;
    fn consume_runtime_chunk<C: RuntimeChunks>(mut self, chunk: C) -> Self::ConsumeRuntimeChunk<C> {
        const { _ = C::NextState::<S>::STATE }
        chunk.runtime_chunks_write_into(&mut self.0);
        ConsumeChunksOfJsonObject(self.0, PhantomData)
    }

    fn end(self) -> Consumed<json_kinds::Object, Self> {
        const {
            S::STATE.assert_eof();
        }
        Consumed::ASSERT_OBJECT
    }
}

mod open_close;

pub struct ConsumeChunksOfNonEmptyArray<
    W: ConsumeTextChunk,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<S>);
pub struct ConsumeChunksOfNonEmptyObject<
    W: ConsumeTextChunk,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
>(W, PhantomData<S>);

impl_many!({
    {
        {
            use ConsumeChunksOfNonEmptyArray as CONSUME;
            use json_kinds::Array as K;

            const fn assert_consumed<W: ?Sized>() -> Consumed<K, W> {
                Consumed::ASSERT_ARRAY
            }

            const fn assert_start(state: State) {
                state.assert_is_top_level_after_array_start()
            }
            const fn assert_end(state: State) {
                state.assert_is_before_top_level_right_bracket()
            }
        }
        {
            use ConsumeChunksOfNonEmptyObject as CONSUME;
            use json_kinds::Object as K;

            const fn assert_consumed<W: ?Sized>() -> Consumed<K, W> {
                Consumed::ASSERT_OBJECT
            }

            const fn assert_start(state: State) {
                state.assert_is_top_level_after_object_start()
            }
            const fn assert_end(state: State) {
                state.assert_is_before_top_level_right_brace()
            }
        }
    }

    impl<W: ConsumeTextChunk, S: ?Sized + HasConstState, const OPEN_CLOSE: u8> ConsumeJsonChunks<K>
        for CONSUME<W, S, OPEN_CLOSE>
    {
        type ConsumeConstChunk<T: ?Sized + HasConstCompileTimeChunk> =
            CONSUME<W, states::NextStateOf<T>, OPEN_CLOSE>;
        fn consume_const_chunk<T: ?Sized + HasConstCompileTimeChunk>(
            mut self,
        ) -> Self::ConsumeConstChunk<T> {
            const {
                S::STATE.assert_same(T::CHUNK.prev_state());

                if T::CHUNK.prev_state().is_init() {
                    assert_start(T::CHUNK.remove_group_open().prev_state());
                }

                if T::CHUNK.next_state().is_eof() {
                    assert_end(T::CHUNK.remove_group_close().next_state());
                }
            }

            if const { <T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.prepend_comma } {
                self.0.consume_text_chunk(",");
            }

            if const { !(<T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.chunk.is_empty()) } {
                self.0
                    .consume_text_chunk(const { <T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.chunk });
            }

            if const { <T as MakeChunks<OPEN_CLOSE>>::MADE_CHUNKS.append_comma } {
                self.0.consume_text_chunk(",");
            }

            CONSUME(self.0, PhantomData)
        }

        type ConsumeRuntimeChunk<C: RuntimeChunks> = CONSUME<W, C::NextState<S>, OPEN_CLOSE>;
        fn consume_runtime_chunk<C: RuntimeChunks>(
            mut self,
            chunk: C,
        ) -> Self::ConsumeRuntimeChunk<C> {
            const { _ = C::NextState::<S>::STATE }
            chunk.runtime_chunks_write_into(&mut self.0);
            CONSUME(self.0, PhantomData)
        }

        fn end(self) -> Consumed<K, Self> {
            const {
                S::STATE.assert_eof();
            }
            assert_consumed()
        }
    }
});

pub struct ConsumeArrayItemsPrependCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);
pub struct ConsumeArrayItemsAppendCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);

impl_many!({
    {
        {
            use ConsumeArrayItemsPrependCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::PREPEND_COMMA;
        }
        {
            use ConsumeArrayItemsAppendCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::APPEND_COMMA;
        }
    }

    impl<W: ConsumeTextChunk> ConsumeJson for CONSUME<W> {
        type ConsumeJsonKind = json_kinds::Array;

        not_string! {}
        not_object! {}

        fn consume_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self> {
            Consumed::ASSERT_ARRAY
        }

        type ConsumeChunksOfNonEmptyArray =
            ConsumeChunksOfNonEmptyArray<W, states::Init, { OC.as_u8() }>;
        fn start_to_consume_chunks_of_non_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
        }

        type ConsumeChainedArrays = Self;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            self
        }
    }

    impl<W: ConsumeTextChunk> ConsumeChainedArrays for CONSUME<W> {
        fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
            let Consumed { .. } =
                arr.json_provide_into(CONSUME(self.0.as_mut_consume_text_chunk()));
        }

        type InitialConsumer = Self;
        fn end_with(
            self,
            arr: impl IntoJson<JsonKind = json_kinds::Array>,
        ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
            arr.json_provide_into(self)
        }
    }
});

pub struct ConsumeObjectKvsPrependCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);
pub struct ConsumeObjectKvsAppendCommaIfNotEmpty<W: ConsumeTextChunk>(pub W);

impl_many!({
    {
        {
            use ConsumeObjectKvsPrependCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::PREPEND_COMMA;
        }
        {
            use ConsumeObjectKvsAppendCommaIfNotEmpty as CONSUME;
            const OC: OpenClose = OpenClose::APPEND_COMMA;
        }
    }

    impl<W: ConsumeTextChunk> ConsumeJson for CONSUME<W> {
        type ConsumeJsonKind = json_kinds::Object;

        not_string! {}
        not_array! {}

        fn consume_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Consumed<json_kinds::Object, Self> {
            Consumed::ASSERT_OBJECT
        }

        type ConsumeChunksOfNonEmptyObject =
            ConsumeChunksOfNonEmptyObject<W, states::Init, { OC.as_u8() }>;

        fn start_to_consume_chunks_of_non_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
        }
    }
});

/// `$(, $item)* ]`
struct ConsumeArrayCommaItemsClose<W: ConsumeTextChunk>(W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayCommaItemsClose<W> {
    type ConsumeJsonKind = json_kinds::Array;

    not_string! {}

    fn consume_empty_array(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk("]");
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray = ConsumeChunksOfNonEmptyArray<
        W,
        states::Init,
        { OpenClose::PREPEND_COMMA_CLOSE_GROUP.as_u8() },
    >;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    type ConsumeChainedArrays = Self;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        self
    }

    fn consume_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        match yes {}
    }

    type ConsumeChunksOfNonEmptyObject = NeverConsume;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        yes: <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        match yes {}
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedArrays for ConsumeArrayCommaItemsClose<W> {
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        let Consumed { .. } = arr.json_provide_into(ConsumeArrayItemsPrependCommaIfNotEmpty(
            self.0.as_mut_consume_text_chunk(),
        ));
    }

    type InitialConsumer = Self; // TODO:
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        // TODO: infinite recursion?
        arr.json_provide_into(self)
    }
}
