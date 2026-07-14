use core::marker::PhantomData;

use crate::ser::{
    IntoJson,
    traits::{ConsumeTextChunk, IntoTextChunks},
};

use super::{
    ConsumeChainedStrings, ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject,
    ConsumeJson, ConsumeJsonText, Consumed,
    consume_chained_content::{ConsumeChainedArrayItems, ConsumeChainedObjectKvs},
    json_kinds::{self, JsonKind},
    json_string_chunks,
    open_close::OpenClose,
    states,
};

pub struct ConsumeStringFragment<W: ConsumeTextChunk>(pub W);

/// TODO: is this needed?
pub struct ConsumeArrayItems<W: ConsumeTextChunk>(pub W);
pub struct ConsumeObjectKvs<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeStringFragment<W> {
    type ConsumeJsonKind = json_kinds::JsonString;
    type Writer = W;

    not_any_value! {}
    not_array! {}
    not_object! {}

    fn consume_empty_string(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        Consumed::ASSERT_STRING
    }
    fn consume_json_string_as_str(
        mut self,
        v: crate::r#const::JsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk(v.fragment());
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

    type EndJsonString = json_string_chunks::EndJsonStringWithNothing;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        mut self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        self.0.consume_text_chunk(v.fragment());
        json_string_chunks::ConsumeInJsonString::new(self.0)
    }
    fn start_to_consume_chunks_of_json_string(
        mut self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        let Consumed { .. } =
            v.json_provide_into(ConsumeStringFragment(self.0.as_mut_consume_text_chunk()));
        json_string_chunks::ConsumeInJsonString::new(self.0)
    }

    type ConsumeChainedStrings = Self;

    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        self
    }
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
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_object! {}

    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        Consumed::ASSERT_ARRAY
    }
    fn consume_non_empty_array_as_str(
        mut self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk(v.items());
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;
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

    fn consume_array_of_items(
        mut self,
        items: impl IntoIterator<Item: IntoJson>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        let mut items = items.into_iter();
        let Some(first) = items.next() else {
            return self.consume_empty_array(());
        };
        first.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
        items.for_each(|item| {
            self.0.consume_text_chunk(",");
            item.json_provide_into(ConsumeJsonText(self.0.as_mut_consume_text_chunk()));
        });
        Consumed::ASSERT_ARRAY
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeObjectKvs<W> {
    type ConsumeJsonKind = json_kinds::Object;
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_array! {}

    fn consume_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        Consumed::ASSERT_OBJECT
    }
    fn consume_non_empty_object_as_str(
        mut self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk(v.kvs());
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject =
        ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;

    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
    }

    type ConsumeChainedObjects = ConsumeChainedObjectKvs<W, bool, Self>;
    fn start_to_consume_chained_objects(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects {
        ConsumeChainedObjectKvs::new_owned(self.0)
    }

    fn consume_object_of_iter(
        mut self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        super::write_kvs(&mut self.0, kvs);
        Consumed::ASSERT_OBJECT
    }
}
