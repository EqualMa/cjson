use crate::{ser::traits::ConsumeTextChunk, utils::impl_many};

use super::{
    ConsumeChainedStrings, ConsumeJsonText, Consumed, IntoJson,
    consume_content::ConsumeStringFragment, consume_content_close::ConsumeStringFragmentClose,
    consume_open_content::ConsumeStringOpenFragmentIfNotEmpty, json_kinds,
};

pub struct ConsumeChainedStringsFull<W: ConsumeTextChunk> {
    writer: W,
    started: bool,
}

pub struct ConsumeChainedArraysFull<W: ConsumeTextChunk> {
    writer: W,
    started: bool,
}

pub struct ConsumeChainedObjectsFull<W: ConsumeTextChunk> {
    writer: W,
    started: bool,
}

impl_many!({
    {
        {
            use ConsumeChainedStringsFull as ConsumeChainedFull;
        }
        {
            use ConsumeChainedArraysFull as ConsumeChainedFull;
        }
        {
            use ConsumeChainedObjectsFull as ConsumeChainedFull;
        }
    }

    impl<W: ConsumeTextChunk> ConsumeChainedFull<W> {
        pub(super) const fn new(writer: W) -> Self {
            Self {
                writer,
                started: false,
            }
        }
    }
});

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

impl_many!({
    {
        {
            use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::ConsumeChainedArrays as TraitConsumeChained;
            use super::consume_comma_content_close::ConsumeArrayCommaItemsClose as ConsumeCommaContentClose;
            use super::consume_open_content::ConsumeArrayOpenItemsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
            use ConsumeChainedArraysFull as ConsumeChainedFull;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeChainedObjects as TraitConsumeChained;
            use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::consume_comma_content_close::ConsumeObjectCommaKvsClose as ConsumeCommaContentClose;
            use super::consume_open_content::ConsumeObjectOpenKvsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
            use ConsumeChainedObjectsFull as ConsumeChainedFull;
            use json_kinds::Object as K;
        }
    }

    impl<W: ConsumeTextChunk> TraitConsumeChained for ConsumeChainedFull<W> {
        fn extend(&mut self, arr: impl IntoJson<JsonKind = K>) {
            if self.started {
                let Consumed { .. } = arr.json_provide_into(ConsumeCommaContent(
                    self.writer.as_mut_consume_text_chunk(),
                ));
            } else {
                let Consumed { .. } = arr.json_provide_into(ConsumeOpenContentIfNotEmpty::new(
                    self.writer.as_mut_consume_text_chunk(),
                    &mut self.started,
                ));
            }
        }

        type InitialConsumer = ConsumeJsonText<W>;
        fn end_with(self, arr: impl IntoJson<JsonKind = K>) -> Consumed<K, Self::InitialConsumer> {
            if self.started {
                let Consumed { .. } = arr.json_provide_into(ConsumeCommaContentClose(self.writer));
                const { Consumed::assert(K) }
            } else {
                arr.json_provide_into(ConsumeJsonText(self.writer))
            }
        }
    }
});
