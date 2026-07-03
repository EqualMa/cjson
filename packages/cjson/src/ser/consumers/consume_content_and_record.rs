use core::marker::PhantomData;

use crate::ser::traits::ConsumeTextChunk;

use super::{
    ConsumeChunksOfNonEmptyArray, ConsumeJson, Consumed,
    consume_chained_content::ConsumeChainedArrayItems,
    json_kinds::{self, JsonKind},
    never_consume::NeverConsume,
    open_close::OpenClose,
    states,
};

pub(super) struct ConsumeArrayItemsAndRecord<'a, W: ConsumeTextChunk> {
    /// should be initialized as false
    started: &'a mut bool,
    writer: W,
}

impl<'a, W: ConsumeTextChunk> ConsumeArrayItemsAndRecord<'a, W> {
    pub(super) const fn new(started: &'a mut bool, writer: W) -> Self {
        debug_assert!(!*started);
        Self { started, writer }
    }
}

impl<'a, W: ConsumeTextChunk> ConsumeJson for ConsumeArrayItemsAndRecord<'a, W> {
    type ConsumeJsonKind = json_kinds::Array;

    not_string! {}

    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        *self.started = true;
        ConsumeChunksOfNonEmptyArray(self.writer, PhantomData)
    }

    type ConsumeChainedArrays = ConsumeChainedArrayItems<W, &'a mut bool, Self>;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        ConsumeChainedArrayItems::new(self.writer, self.started)
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
