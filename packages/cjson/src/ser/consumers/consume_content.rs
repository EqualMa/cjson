use core::marker::PhantomData;

use crate::ser::{
    IntoJson,
    traits::{ConsumeTextChunk, IntoTextChunks},
};

use super::{
    ConsumeChainedStrings, ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject,
    ConsumeJson, Consumed,
    consume_chained_content::ConsumeChainedArrayItems,
    json_kinds::{self, JsonKind},
    open_close::OpenClose,
    states,
};

pub struct ConsumeStringFragment<W: ConsumeTextChunk>(pub W);

/// TODO: is this needed?
pub struct ConsumeArrayItems<W: ConsumeTextChunk>(pub W);
pub struct ConsumeObjectKvs<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeStringFragment<W> {
    type ConsumeJsonKind = json_kinds::JsonString;

    fn consume_empty_string(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        Consumed::ASSERT_STRING
    }

    fn consume_str(
        mut self,
        s: &str,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        crate::ser::texts::StrToJsonStringFragment(s).write_into(&mut self.0);
        Consumed::ASSERT_STRING
    }

    type ConsumeChainedStrings = Self;

    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        self
    }

    not_array! {}
    not_object! {}
}

impl<W: ConsumeTextChunk> ConsumeChainedStrings for ConsumeStringFragment<W> {
    fn extend(&mut self, s: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        let Consumed { .. } =
            s.json_provide_into(ConsumeStringFragment(self.0.as_mut_consume_text_chunk()));
    }

    type InitialConsumer = Self;
    fn end_with(
        self,
        s: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        s.json_provide_into(self)
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayItems<W> {
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
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
    }

    type ConsumeChainedArrays = ConsumeChainedArrayItems<W, bool, Self>;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        ConsumeChainedArrayItems::new_owned(self.0)
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeObjectKvs<W> {
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
        ConsumeChunksOfNonEmptyObject<W, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;

    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
    }
}
