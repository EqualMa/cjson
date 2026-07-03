use crate::ser::traits::ConsumeTextChunk;

use super::{
    ConsumeArrayCommaItemsClose, ConsumeArrayItemsPrependCommaIfNotEmpty,
    ConsumeArrayOpenItemsIfNotEmpty, ConsumeChainedArrays, ConsumeChainedStrings, ConsumeJsonText,
    Consumed, IntoJson, consume_content::ConsumeStringFragment,
    consume_content_close::ConsumeStringFragmentClose,
    consume_open_content::ConsumeStringOpenFragmentIfNotEmpty, json_kinds,
};

pub struct ConsumeChainedStringsFull<W: ConsumeTextChunk> {
    writer: W,
    started: bool,
}

impl<W: ConsumeTextChunk> ConsumeChainedStringsFull<W> {
    pub(super) fn new(writer: W) -> Self {
        Self {
            writer,
            started: false,
        }
    }
}

pub struct ConsumeChainedArraysFull<W: ConsumeTextChunk> {
    writer: W,
    started: bool,
}

impl<W: ConsumeTextChunk> ConsumeChainedArraysFull<W> {
    pub(super) const fn new(writer: W) -> Self {
        Self {
            writer,
            started: false,
        }
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedStrings for ConsumeChainedStringsFull<W> {
    fn extend(&mut self, s: impl IntoJson<JsonKind = json_kinds::JsonString>) {
        if self.started {
            let Consumed { .. } = s.json_provide_into(ConsumeStringFragment(
                self.writer.as_mut_consume_text_chunk(),
            ));
        } else {
            let Consumed { .. } = s.json_provide_into(ConsumeStringOpenFragmentIfNotEmpty::new(
                self.writer.as_mut_consume_text_chunk(),
                &mut self.started,
            ));
        }
    }

    type InitialConsumer = ConsumeJsonText<W>;
    fn end_with(
        self,
        s: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        if self.started {
            let Consumed { .. } = s.json_provide_into(ConsumeStringFragmentClose(self.writer));
            Consumed::ASSERT_STRING
        } else {
            s.json_provide_into(ConsumeJsonText(self.writer))
        }
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedArrays for ConsumeChainedArraysFull<W> {
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        if self.started {
            let Consumed { .. } = arr.json_provide_into(ConsumeArrayItemsPrependCommaIfNotEmpty(
                self.writer.as_mut_consume_text_chunk(),
            ));
        } else {
            let Consumed { .. } = arr.json_provide_into(ConsumeArrayOpenItemsIfNotEmpty::new(
                self.writer.as_mut_consume_text_chunk(),
                &mut self.started,
            ));
        }
    }

    type InitialConsumer = ConsumeJsonText<W>;
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        if self.started {
            let Consumed { .. } = arr.json_provide_into(ConsumeArrayCommaItemsClose(self.writer));
            Consumed::ASSERT_ARRAY
        } else {
            arr.json_provide_into(ConsumeJsonText(self.writer))
        }
    }
}
