use core::marker::PhantomData;

use crate::{
    r#const::states,
    ser::{IntoJson, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{
    ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject, ConsumeJson, Consumed,
    json_kinds::{self, JsonKind},
    open_close::OpenClose,
};

/// `$(, $item)* ]`
pub(super) struct ConsumeArrayCommaItemsClose<W: ConsumeTextChunk>(pub W);
pub(super) struct ConsumeObjectCommaKvsClose<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayCommaItemsClose<W> {
    type ConsumeJsonKind = json_kinds::Array;
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_object! {}

    fn consume_empty_array(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_text_chunk("]");
        Consumed::ASSERT_ARRAY
    }
    fn consume_non_empty_array_as_str(
        mut self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        self.0.consume_2_text_chunks(",", v.items_close());
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray = ConsumeChunksOfNonEmptyArray<
        W,
        Self,
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

    fn consume_array_of_items(
        mut self,
        items: impl IntoIterator<Item: IntoJson>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        let Consumed { .. } =
            super::ConsumeArrayItemsPrependCommaIfNotEmpty(self.0.as_mut_consume_text_chunk())
                .consume_array_of_items(items, ());

        self.0.consume_text_chunk("]");

        Consumed::ASSERT_ARRAY
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeObjectCommaKvsClose<W> {
    type ConsumeJsonKind = json_kinds::Object;
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_array! {}

    fn consume_empty_object(
        mut self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_text_chunk("}");
        Consumed::ASSERT_OBJECT
    }
    fn consume_non_empty_object_as_str(
        mut self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        self.0.consume_2_text_chunks(",", v.kvs_close());
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject = ConsumeChunksOfNonEmptyObject<
        W,
        Self,
        states::Init,
        { OpenClose::PREPEND_COMMA_CLOSE_GROUP.as_u8() },
    >;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
    }

    type ConsumeChainedObjects = Self;
    fn start_to_consume_chained_objects(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects {
        self
    }

    fn consume_object_of_iter(
        mut self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        let Consumed { .. } =
            super::ConsumeObjectKvsPrependCommaIfNotEmpty(self.0.as_mut_consume_text_chunk())
                .consume_object_of_iter(kvs, ());

        self.0.consume_text_chunk("}");

        Consumed::ASSERT_OBJECT
    }
}

impl_many!({
    {
        {
            use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use ConsumeArrayCommaItemsClose as ConsumeCommaContentClose;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use ConsumeObjectCommaKvsClose as ConsumeCommaContentClose;
            use json_kinds::Object as K;
        }
    }

    impl<W: ConsumeTextChunk> super::ConsumeChained<K> for ConsumeCommaContentClose<W> {
        fn extend<V: IntoJson<JsonKind = K>>(&mut self, arr: V) {
            let Consumed { .. } =
                arr.json_provide_into(ConsumeCommaContent(self.0.as_mut_consume_text_chunk()));
        }

        type InitialConsumer = Self; // TODO:
        fn end_with<V: IntoJson<JsonKind = K>>(
            self,
            content: V,
        ) -> Consumed<K, Self::InitialConsumer> {
            // TODO: infinite recursion?
            content.json_provide_into(self)
        }
    }
});
