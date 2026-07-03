use core::marker::PhantomData;

use crate::ser::{IntoJson, traits::ConsumeTextChunk};

use super::{
    ConsumeArrayItemsPrependCommaIfNotEmpty, ConsumeChainedArrays, ConsumeChainedStrings,
    ConsumeChunksOfNonEmptyArray, ConsumeJson, Consumed,
    consume_content::ConsumeStringFragment,
    json_kinds::{self, JsonKind},
    open_close::OpenClose,
    states,
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

pub(super) struct ConsumeStringOpenFragmentIfNotEmpty<'a, W: ConsumeTextChunk> {
    writer: W,
    started: &'a mut bool,
}

impl<'a, W: ConsumeTextChunk> ConsumeArrayOpenItemsIfNotEmpty<'a, W> {
    pub(super) fn new(writer: W, started: &'a mut bool) -> Self {
        debug_assert!(!*started);
        Self { writer, started }
    }
}

impl<'a, W: ConsumeTextChunk> ConsumeStringOpenFragmentIfNotEmpty<'a, W> {
    pub(super) fn new(writer: W, started: &'a mut bool) -> Self {
        debug_assert!(!*started);
        Self { writer, started }
    }
}

impl<W: ConsumeTextChunk> ConsumeArrayOpenItemsIfNotEmpty<'_, W> {
    fn impl_extend(self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        if *self.started {
            let Consumed { .. } =
                arr.json_provide_into(ConsumeArrayItemsPrependCommaIfNotEmpty(self.writer));
        } else {
            // TODO: infinite recursion?
            let Consumed { .. } = arr.json_provide_into(self);
        }
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeArrayOpenItemsIfNotEmpty<'_, W> {
    type ConsumeJsonKind = json_kinds::Array;

    not_any_value! {}
    not_string! {}
    not_object! {}

    fn consume_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self> {
        Consumed::ASSERT_ARRAY
    }

    type ConsumeChunksOfNonEmptyArray =
        ConsumeChunksOfNonEmptyArray<W, states::Init, { OpenClose::OPEN_GROUP.as_u8() }>;
    fn start_to_consume_chunks_of_non_empty_array(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChunksOfNonEmptyArray {
        *self.started = true;
        ConsumeChunksOfNonEmptyArray(self.writer, PhantomData)
    }

    type ConsumeChainedArrays = Self;
    fn start_to_consume_chained_arrays(
        self,
        (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
    ) -> Self::ConsumeChainedArrays {
        self
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedArrays for ConsumeArrayOpenItemsIfNotEmpty<'_, W> {
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        ConsumeArrayOpenItemsIfNotEmpty {
            writer: self.writer.as_mut_consume_text_chunk(),
            started: self.started,
        }
        .impl_extend(arr)
    }

    type InitialConsumer = Self;
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        self.impl_extend(arr);
        Consumed::ASSERT_ARRAY
    }
}

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeStringOpenFragmentIfNotEmpty<'_, W> {
    type ConsumeJsonKind = json_kinds::JsonString;

    not_any_value! {}
    not_object! {}
    not_array! {}

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
        if !s.is_empty() {
            if !*self.started {
                *self.started = true;
                self.writer.consume_text_chunk("\"");
            }
            let Consumed { .. } = ConsumeStringFragment(self.writer).consume_str(s, ());
        }

        Consumed::ASSERT_STRING
    }

    type ConsumeChainedStrings = Self;

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
    fn extend(&mut self, s: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        ConsumeStringOpenFragmentIfNotEmpty {
            writer: self.writer.as_mut_consume_text_chunk(),
            started: self.started,
        }
        .impl_extend(s);
    }

    type InitialConsumer = Self;
    fn end_with(
        self,
        s: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        self.impl_extend(s);
        Consumed::ASSERT_STRING
    }
}
