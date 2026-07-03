use core::{borrow::BorrowMut, marker::PhantomData};

use crate::ser::traits::ConsumeTextChunk;

use super::{
    ConsumeArrayItemsPrependCommaIfNotEmpty, ConsumeChainedArrays, Consumed, IntoJson,
    consume_content::ConsumeArrayItems, consume_content_and_record::ConsumeArrayItemsAndRecord,
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

pub trait ConsumeChainedArrayItemsInitialConsumer {}

impl<W: ConsumeTextChunk> ConsumeChainedArrayItemsInitialConsumer for ConsumeArrayItems<W> {}
impl<W: ConsumeTextChunk> ConsumeChainedArrayItemsInitialConsumer
    for ConsumeArrayItemsAndRecord<'_, W>
{
}

impl<W: ConsumeTextChunk, I: ConsumeChainedArrayItemsInitialConsumer>
    ConsumeChainedArrayItems<W, bool, I>
{
    pub(super) fn new_owned(writer: W) -> Self {
        Self {
            writer,
            started: false,
            initial_consumer: PhantomData,
        }
    }
}

impl<W: ConsumeTextChunk, S: BorrowMut<bool>, I: ConsumeChainedArrayItemsInitialConsumer>
    ConsumeChainedArrayItems<W, S, I>
{
    pub(super) fn new(writer: W, mut started: S) -> Self {
        debug_assert!(!*started.borrow_mut());
        Self {
            writer,
            started,
            initial_consumer: PhantomData,
        }
    }

    fn impl_extend(mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        let started = self.started.borrow_mut();
        if *started {
            let Consumed { .. } =
                arr.json_provide_into(ConsumeArrayItemsPrependCommaIfNotEmpty(self.writer));
        } else {
            let Consumed { .. } =
                arr.json_provide_into(ConsumeArrayItemsAndRecord::new(started, self.writer));
        }
    }
}

impl<W: ConsumeTextChunk, S: BorrowMut<bool>, I: ConsumeChainedArrayItemsInitialConsumer>
    ConsumeChainedArrays for ConsumeChainedArrayItems<W, S, I>
{
    fn extend(&mut self, arr: impl IntoJson<JsonKind = json_kinds::Array>) {
        ConsumeChainedArrayItems {
            writer: self.writer.as_mut_consume_text_chunk(),
            started: self.started.borrow_mut(),
            initial_consumer: PhantomData::<I>,
        }
        .impl_extend(arr)
    }

    type InitialConsumer = I;
    fn end_with(
        self,
        arr: impl IntoJson<JsonKind = json_kinds::Array>,
    ) -> Consumed<json_kinds::Array, Self::InitialConsumer> {
        self.impl_extend(arr);
        Consumed::ASSERT_ARRAY
    }
}
