use crate::ser::traits::{ConsumeTextChunk, IntoTextChunks};

use super::{
    ConsumeChainedStrings, ConsumeJson, Consumed, consume_content::ConsumeStringFragment,
    json_kinds, json_string_chunks,
};

pub(super) struct ConsumeStringFragmentClose<W: ConsumeTextChunk>(pub W);

impl<W: ConsumeTextChunk> ConsumeJson for ConsumeStringFragmentClose<W> {
    type ConsumeJsonKind = json_kinds::JsonString;
    type Writer = W;

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

    fn consume_json_string_as_str(
        mut self,
        v: crate::r#const::JsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> Consumed<json_kinds::JsonString, Self> {
        self.0.consume_text_chunk(v.fragment_close());
        Consumed::ASSERT_STRING
    }

    type EndJsonString = json_string_chunks::EndJsonStringWithClose;
    fn start_to_consume_chunks_of_json_string_with_first_chunk(
        mut self,
        v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        self.0.consume_text_chunk(v.fragment());
        json_string_chunks::ConsumeInJsonString::new(self.0)
    }
    fn start_to_consume_chunks_of_json_string(
        mut self,
        v: impl crate::ser::IntoJson<JsonKind = json_kinds::JsonString>,
        (): <Self::ConsumeJsonKind as json_kinds::JsonKind>::Contains<json_kinds::JsonString>,
    ) -> json_string_chunks::ConsumeInJsonString<Self::EndJsonString, Self> {
        let Consumed { .. } = v.json_provide_into(super::consume_content::ConsumeStringFragment(
            self.0.as_mut_consume_text_chunk(),
        ));
        json_string_chunks::ConsumeInJsonString::new(self.0)
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
    fn extend<V: crate::ser::IntoJson<JsonKind = json_kinds::JsonString>>(&mut self, s: V) {
        let Consumed { .. } =
            s.json_provide_into(ConsumeStringFragment(self.0.as_mut_consume_text_chunk()));
    }

    type InitialConsumer = Self;
    fn end_with<V: crate::ser::IntoJson<JsonKind = json_kinds::JsonString>>(
        self,
        s: V,
    ) -> Consumed<json_kinds::JsonString, Self::InitialConsumer> {
        s.json_provide_into(self)
    }
}
