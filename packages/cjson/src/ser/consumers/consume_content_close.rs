use crate::ser::traits::{ConsumeTextChunk, IntoTextChunks};

use super::{
    ConsumeChainedStrings, ConsumeJson, Consumed, consume_content::ConsumeStringFragment,
    json_kinds,
};

pub(super) struct ConsumeStringFragmentClose<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeStringFragmentClose<W> {
    type ConsumeJsonKind = json_kinds::JsonString;

    not_any_value! {}
    not_array! {}
    not_object! {}

    fn consume_empty_string(
        mut self,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk("\"");
        Consumed::ASSERT_STRING
    }

    fn consume_str(
        mut self,
        s: &str,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        crate::ser::texts::StrToJsonStringFragment(s).write_into(&mut self.0);
        self.0.consume_text_chunk("\"");
        Consumed::ASSERT_STRING
    }

    type ConsumeChainedStrings = Self;
    fn start_to_consume_chained_strings(
        self,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Self::ConsumeChainedStrings {
        self
    }
}

impl<W: ConsumeTextChunk> ConsumeChainedStrings for ConsumeStringFragmentClose<W> {
    fn extend(&mut self, s: impl crate::ser::IntoJson<JsonKind = json_kinds::JsonString>) {
        let Consumed { .. } =
            s.json_provide_into(ConsumeStringFragment(self.0.as_mut_consume_text_chunk()));
    }

    type InitialConsumer = Self;
    fn end_with(
        self,
        s: impl crate::ser::IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        s.json_provide_into(self)
    }
}
