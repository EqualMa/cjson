use core::marker::PhantomData;

use crate::{
    ser::{IntoJson, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{
    ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject, ConsumeJson, ConsumeJsonText,
    Consumed,
    consume_chained_content::{ConsumeChainedArrayItems, ConsumeChainedObjectKvs},
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

pub(super) struct ConsumeObjectKvsAndRecord<'a, W: ConsumeTextChunk> {
    /// should be initialized as false
    started: &'a mut bool,
    writer: W,
}

impl_many!({
    {
        {
            use ConsumeArrayItemsAndRecord as ConsumeContentAndRecord;
        }
        {
            use ConsumeObjectKvsAndRecord as ConsumeContentAndRecord;
        }
    }

    impl<'a, W: ConsumeTextChunk> ConsumeContentAndRecord<'a, W> {
        pub(super) const fn new(started: &'a mut bool, writer: W) -> Self {
            debug_assert!(!*started);
            Self { started, writer }
        }
    }
});

impl<'a, W: ConsumeTextChunk> ConsumeJson for ConsumeArrayItemsAndRecord<'a, W> {
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
        *self.started = true;
        self.writer.consume_text_chunk(v.items());
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;
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

    fn consume_array_of_items(
        mut self,
        items: impl IntoIterator<Item: crate::ser::IntoJson>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        let mut items = items.into_iter();
        let Some(first) = items.next() else {
            return self.consume_empty_array(());
        };
        *self.started = true;
        first.json_provide_into(ConsumeJsonText(self.writer.as_mut_consume_text_chunk()));
        items.for_each(|item| {
            self.writer.consume_text_chunk(",");
            item.json_provide_into(ConsumeJsonText(self.writer.as_mut_consume_text_chunk()));
        });
        Consumed::ASSERT_ARRAY
    }
}

impl<'a, W: ConsumeTextChunk> ConsumeJson for ConsumeObjectKvsAndRecord<'a, W> {
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
        *self.started = true;
        self.writer.consume_text_chunk(v.kvs());
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject =
        ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OpenClose::BOTH_NOTHING.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        *self.started = true;
        ConsumeChunksOfNonEmptyObject(self.writer, PhantomData)
    }

    type ConsumeChainedObjects = ConsumeChainedObjectKvs<W, &'a mut bool, Self>;
    fn start_to_consume_chained_objects(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects {
        ConsumeChainedObjectKvs::new(self.writer, self.started)
    }

    fn consume_object_of_iter(
        mut self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        let mut kvs = kvs.into_iter();
        let Some(first) = kvs.next() else {
            return self.consume_empty_object(());
        };
        *self.started = true;
        super::write_non_empty_kvs(&mut self.writer, first, kvs);
        Consumed::ASSERT_OBJECT
    }
}
