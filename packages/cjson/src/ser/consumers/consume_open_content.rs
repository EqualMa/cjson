use core::marker::PhantomData;

use crate::{
    ser::{IntoJson, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{
    ConsumeChainedStrings, ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject,
    ConsumeJson, ConsumeJsonText, Consumed,
    consume_content::ConsumeStringFragment,
    json_kinds::{self, JsonKind},
    json_string_chunks,
    open_close::OpenClose,
    states, write_comma_kvs, write_key_frag_quote_colon_value,
};

/// - If the array contains no item, consumes nothing.
/// - Else, the array contains at least one item, then:
///   - If `*self.started`, consumes comma and comma separated items.
///   - Else, set `*self.started` to `true`, consumes `[` and comma separated items.
///
/// The above is equivalent to the following:
///
/// - If `*self.started`: consumes `$( , $item )*`.
/// - Else:
///   - If the array contains no item, consumes nothing.
///   - Else, set `*self.started` to `true`, consumes `[` and comma separated items.
pub(super) struct ConsumeArrayOpenItemsIfNotEmpty<'a, W: ConsumeTextChunk> {
    writer: W,
    /// TODO: refactor [`Consumed`] to include payload as `<W as ConsumeJson>::ConsumeJsonPayload`.
    started: &'a mut bool,
}

pub(super) struct ConsumeObjectOpenKvsIfNotEmpty<'a, W: ConsumeTextChunk> {
    writer: W,
    /// TODO: refactor [`Consumed`] to include payload as `<W as ConsumeJson>::ConsumeJsonPayload`.
    started: &'a mut bool,
}

pub(super) struct ConsumeStringOpenFragmentIfNotEmpty<'a, W: ConsumeTextChunk> {
    writer: W,
    started: &'a mut bool,
}

impl_many!({
    {
        {
            use ConsumeArrayOpenItemsIfNotEmpty as This;
        }
        {
            use ConsumeObjectOpenKvsIfNotEmpty as This;
        }
        {
            use ConsumeStringOpenFragmentIfNotEmpty as This;
        }
    }
    impl<'a, W: ConsumeTextChunk> This<'a, W> {
        pub(super) fn new(writer: W, started: &'a mut bool) -> Self {
            debug_assert!(!*started);
            Self { writer, started }
        }
    }
});

impl_many!({
    {
        {
            use super::ConsumeArrayItemsPrependCommaIfNotEmpty as TConsumeCommaContent;
            use ConsumeArrayOpenItemsIfNotEmpty as This;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeObjectKvsPrependCommaIfNotEmpty as TConsumeCommaContent;
            use ConsumeObjectOpenKvsIfNotEmpty as This;
            use json_kinds::Object as K;
        }
    }

    impl<W: ConsumeTextChunk> This<'_, W> {
        fn impl_extend(self, arr: impl IntoJson<JsonKind = K>) {
            if *self.started {
                let Consumed { .. } = arr.json_provide_into(TConsumeCommaContent(self.writer));
            } else {
                // TODO: infinite recursion?
                let Consumed { .. } = arr.json_provide_into(self);
            }
        }
    }
});

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayOpenItemsIfNotEmpty<'_, W> {
    type ConsumeJsonKind = json_kinds::Array;
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_object! {}

    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        debug_assert!(!*self.started);
        Consumed::ASSERT_ARRAY
    }
    fn consume_non_empty_array_as_str(
        mut self,
        v: crate::r#const::NonEmptyArrayAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        debug_assert!(!*self.started);
        *self.started = true;
        self.writer.consume_text_chunk(v.open_items());
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, Self, states::Init, { OpenClose::OPEN_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        debug_assert!(!*self.started);
        *self.started = true;
        ConsumeChunksOfNonEmptyArray(self.writer, PhantomData)
    }

    type ConsumeChainedArrays = Self;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        debug_assert!(!*self.started);
        self
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

        debug_assert!(!*self.started);
        *self.started = true;
        self.writer.consume_text_chunk("[");
        first.json_provide_into(ConsumeJsonText(self.writer.as_mut_consume_text_chunk()));
        items.for_each(|item| {
            self.writer.consume_text_chunk(",");
            item.json_provide_into(ConsumeJsonText(self.writer.as_mut_consume_text_chunk()));
        });
        Consumed::ASSERT_ARRAY
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeObjectOpenKvsIfNotEmpty<'_, W> {
    type ConsumeJsonKind = json_kinds::Object;
    type Writer = W;

    not_any_value! {}
    not_string! {}
    not_array! {}

    fn consume_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        debug_assert!(!*self.started);
        Consumed::ASSERT_OBJECT
    }
    fn consume_non_empty_object_as_str(
        mut self,
        v: crate::r#const::NonEmptyObjectAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        debug_assert!(!*self.started);
        *self.started = true;
        self.writer.consume_text_chunk(v.open_kvs());
        Consumed::ASSERT_OBJECT
    }

    type ConsumeChunksOfNonEmptyObject =
        ConsumeChunksOfNonEmptyObject<W, Self, states::Init, { OpenClose::OPEN_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_object(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChunksOfNonEmptyObject {
        debug_assert!(!*self.started);
        *self.started = true;
        ConsumeChunksOfNonEmptyObject(self.writer, PhantomData)
    }

    type ConsumeChainedObjects = Self;
    fn start_to_consume_chained_objects(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Self::ConsumeChainedObjects {
        debug_assert!(!*self.started);
        self
    }

    fn consume_object_of_iter(
        mut self,
        kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
    ) -> Consumed<json_kinds::Object, Self> {
        // ["key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value
        let mut kvs = kvs.into_iter();
        let Some(first) = kvs.next() else {
            return self.consume_empty_object(());
        };

        debug_assert!(!*self.started);
        self.writer.consume_text_chunk("[\""); // TODO: ConsumeStringOpenFragment
        write_key_frag_quote_colon_value(&mut self.writer, first);

        write_comma_kvs(&mut self.writer, kvs);

        Consumed::ASSERT_OBJECT
    }
}

impl_many!({
    {
        {
            use super::ConsumeChainedArrays as TraitConsumeChained;
            use ConsumeArrayOpenItemsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeChainedObjects as TraitConsumeChained;
            use ConsumeObjectOpenKvsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
            use json_kinds::Object as K;
        }
    }

    impl<W: ConsumeTextChunk> TraitConsumeChained for ConsumeOpenContentIfNotEmpty<'_, W> {
        fn extend<V: IntoJson<JsonKind = K>>(&mut self, content: V) {
            ConsumeOpenContentIfNotEmpty {
                writer: self.writer.as_mut_consume_text_chunk(),
                started: self.started,
            }
            .impl_extend(content)
        }

        type InitialConsumer = Self;
        fn end_with<V: IntoJson<JsonKind = K>>(
            self,
            content: V,
        ) -> Consumed<K, Self::InitialConsumer> {
            self.impl_extend(content);
            const { Consumed::assert(K) }
        }
    }
});

impl<'a, W: ConsumeTextChunk> ConsumeJson for ConsumeStringOpenFragmentIfNotEmpty<'a, W> {
    type ConsumeJsonKind = json_kinds::JsonString;
    type Writer = W;

    not_any_value! {}
    not_object! {}
    not_array! {}

    fn consume_empty_string(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        debug_assert!(!*self.started);
        Consumed::ASSERT_STRING
    }

    fn consume_json_string_as_str(
        mut self,
        v: crate::r#const::JsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        let Some(open_non_empty_fragment) = v.open_non_empty_fragment() else {
            return self.consume_empty_string(());
        };

        debug_assert!(!*self.started);
        *self.started = true;

        self.writer.consume_text_chunk(open_non_empty_fragment);

        Consumed::ASSERT_STRING
    }

    fn consume_str(
        mut self,
        s: &str,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        debug_assert!(!*self.started);

        if !s.is_empty() {
            *self.started = true;

            // TODO: optimize with ConsumeStringOpenFragment
            self.writer.consume_text_chunk("\"");
            let Consumed { .. } = ConsumeStringFragment(self.writer).consume_str(s, ());
        }

        Consumed::ASSERT_STRING
    }

    type EndJsonString = json_string_chunks::EndJsonStringOpenFragmentIfNotEmpty<'a>;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        mut self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        debug_assert!(!*self.started);

        'open: {
            let Some(open_non_empty_fragment) = v.open_non_empty_fragment() else {
                break 'open;
            };

            *self.started = true;
            self.writer.consume_text_chunk(open_non_empty_fragment);
        }

        json_string_chunks::ConsumeInJsonString::new_full(
            json_string_chunks::EndJsonStringOpenFragmentIfNotEmpty {
                started: self.started,
            },
            self.writer,
        )
    }
    fn start_to_consume_chunks_of_json_string(
        mut self,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        debug_assert!(!*self.started);

        // TODO: Will this cause infinite recursion?
        let Consumed { .. } = v.json_provide_into(ConsumeStringOpenFragmentIfNotEmpty {
            writer: self.writer.as_mut_consume_text_chunk(),
            started: self.started,
        });

        json_string_chunks::ConsumeInJsonString::new_full(
            json_string_chunks::EndJsonStringOpenFragmentIfNotEmpty {
                started: self.started,
            },
            self.writer,
        )
    }

    type ConsumeChainedStrings = Self;
    // TODO: Self should wrap ConsumeChainedStrings. ConsumeChainedStrings allows started to be true

    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        self
    }
}

impl<W: ConsumeTextChunk> ConsumeStringOpenFragmentIfNotEmpty<'_, W> {
    fn impl_extend(self, s: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        if *self.started {
            let Consumed { .. } = s.json_provide_into(ConsumeStringFragment(self.writer));
        } else {
            let Consumed { .. } = s.json_provide_into(self);
        }
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedStrings for ConsumeStringOpenFragmentIfNotEmpty<'_, W> {
    fn extend<V: IntoJson<JsonKind = json_kinds::JsonString>>(&mut self, s: V) {
        ConsumeStringOpenFragmentIfNotEmpty {
            writer: self.writer.as_mut_consume_text_chunk(),
            started: self.started,
        }
        .impl_extend(s);
    }

    type InitialConsumer = Self;
    fn end_with<V: IntoJson<JsonKind = json_kinds::JsonString>>(
        self,
        s: V,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        self.impl_extend(s);
        Consumed::ASSERT_STRING
    }
}
