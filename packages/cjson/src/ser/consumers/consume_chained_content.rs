use core::{borrow::BorrowMut, marker::PhantomData};

use crate::{ser::traits::ConsumeTextChunk, utils::impl_many};

use super::{
    Consumed, IntoJson,
    consume_content::{ConsumeArrayItems, ConsumeObjectKvs},
    consume_content_and_record::{ConsumeArrayItemsAndRecord, ConsumeObjectKvsAndRecord},
    json_kinds,
};

pub struct ConsumeChainedArrayItems<
    W: ConsumeTextChunk,
    S: BorrowMut<bool>,
    I: ConsumeChainedArrayItemsInitialConsumer,
> {
    writer: W,
    started: S,
    initial_consumer: PhantomData<I>,
}

pub struct ConsumeChainedObjectKvs<
    W: ConsumeTextChunk,
    S: BorrowMut<bool>,
    I: ConsumeChainedObjectKvsInitialConsumer,
> {
    writer: W,
    started: S,
    initial_consumer: PhantomData<I>,
}

pub trait ConsumeChainedArrayItemsInitialConsumer {}

impl<W: ConsumeTextChunk> ConsumeChainedArrayItemsInitialConsumer for ConsumeArrayItems<W> {}
impl<W: ConsumeTextChunk> ConsumeChainedArrayItemsInitialConsumer
    for ConsumeArrayItemsAndRecord<'_, W>
{
}

pub trait ConsumeChainedObjectKvsInitialConsumer {}

impl<W: ConsumeTextChunk> ConsumeChainedObjectKvsInitialConsumer for ConsumeObjectKvs<W> {}
impl<W: ConsumeTextChunk> ConsumeChainedObjectKvsInitialConsumer
    for ConsumeObjectKvsAndRecord<'_, W>
{
}

impl_many!({
    {
        {
            use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::ConsumeChainedArrays as TraitConsumeChained;
            use super::consume_content_and_record::ConsumeArrayItemsAndRecord as ConsumeContentAndRecord;
            use ConsumeChainedArrayItems as ConsumeChainedContent;
            use ConsumeChainedArrayItemsInitialConsumer as TraitInitialConsumer;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeChainedObjects as TraitConsumeChained;
            use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::consume_content_and_record::ConsumeObjectKvsAndRecord as ConsumeContentAndRecord;
            use ConsumeChainedObjectKvs as ConsumeChainedContent;
            use ConsumeChainedObjectKvsInitialConsumer as TraitInitialConsumer;
            use json_kinds::Object as K;
        }
    }

    impl<W: ConsumeTextChunk, I: TraitInitialConsumer> ConsumeChainedContent<W, bool, I> {
        pub(super) fn new_owned(writer: W) -> Self {
            Self {
                writer,
                started: false,
                initial_consumer: PhantomData,
            }
        }
    }

    impl<W: ConsumeTextChunk, S: BorrowMut<bool>, I: TraitInitialConsumer>
        ConsumeChainedContent<W, S, I>
    {
        pub(super) fn new(writer: W, mut started: S) -> Self {
            debug_assert!(!*started.borrow_mut());
            Self {
                writer,
                started,
                initial_consumer: PhantomData,
            }
        }

        fn impl_extend(mut self, arr: impl IntoJson<JsonKind = K>) {
            let started = self.started.borrow_mut();
            if *started {
                let Consumed { .. } = arr.json_provide_into(ConsumeCommaContent(self.writer));
            } else {
                let Consumed { .. } =
                    arr.json_provide_into(ConsumeContentAndRecord::new(started, self.writer));
            }
        }
    }

    impl<W: ConsumeTextChunk, S: BorrowMut<bool>, I: TraitInitialConsumer> TraitConsumeChained
        for ConsumeChainedContent<W, S, I>
    {
        fn extend(&mut self, arr: impl IntoJson<JsonKind = K>) {
            ConsumeChainedContent {
                writer: self.writer.as_mut_consume_text_chunk(),
                started: self.started.borrow_mut(),
                initial_consumer: PhantomData::<I>,
            }
            .impl_extend(arr)
        }

        type InitialConsumer = I;
        fn end_with(self, arr: impl IntoJson<JsonKind = K>) -> Consumed<K, Self::InitialConsumer> {
            self.impl_extend(arr);
            Consumed::assert(K)
        }
    }
});
