use core::marker::PhantomData;

use crate::{
    r#const::State,
    ser::{Consumed, IntoJson, consumers::HasConstState, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::json_kinds;

pub struct ConsumeArrayOpenContentComma<
    W: ConsumeTextChunk,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
> {
    writer: W,
    started: bool,
    initial_consumer: PhantomData<(InitialConsumer, S)>,
}
pub struct ConsumeObjectOpenContentComma<
    W: ConsumeTextChunk,
    InitialConsumer,
    S: ?Sized + HasConstState,
    const OPEN_CLOSE: u8,
> {
    writer: W,
    started: bool,
    initial_consumer: PhantomData<(InitialConsumer, S)>,
}

enum Never {}
pub struct ArrayNextStateOf<S: ?Sized + HasConstState>(Never, PhantomData<S>);
pub struct ObjectNextStateOf<S: ?Sized + HasConstState>(Never, PhantomData<S>);

impl<S: ?Sized + HasConstState> HasConstState for ArrayNextStateOf<S> {
    const STATE: State = S::STATE
        .left_bracket()
        .json_items_after_array_start_before_item();
}

impl<S: ?Sized + HasConstState> HasConstState for ObjectNextStateOf<S> {
    const STATE: State = S::STATE
        .left_brace()
        .json_kvs_after_object_start_before_kv();
}

impl_many!({
    {
        {
            use self::ArrayNextStateOf as NextStateOf;
            use self::ConsumeArrayOpenContentComma as TConsumeOpenContentComma;
            use super::ConsumeArrayItemsAppendCommaIfNotEmpty as TConsumeContentComma;
            use super::ConsumeChunksOfNonEmptyArray as TConsumeChunksOfNonEmpty;
            use super::consume_open_content::ConsumeArrayOpenItemsIfNotEmpty as TConsumeOpenItemsIfNotEmpty;
            use json_kinds::Array as K;

            const OPEN: &str = "[";
        }
        {
            use self::ConsumeObjectOpenContentComma as TConsumeOpenContentComma;
            use self::ObjectNextStateOf as NextStateOf;
            use super::ConsumeChunksOfNonEmptyObject as TConsumeChunksOfNonEmpty;
            use super::ConsumeObjectKvsAppendCommaIfNotEmpty as TConsumeContentComma;
            use super::consume_open_content::ConsumeObjectOpenKvsIfNotEmpty as TConsumeOpenItemsIfNotEmpty; // TODO:
            use json_kinds::Object as K;

            const OPEN: &str = "{";
        }
    }

    impl<W: ConsumeTextChunk, InitialConsumer, S: ?Sized + HasConstState, const OPEN_CLOSE: u8>
        TConsumeOpenContentComma<W, InitialConsumer, S, OPEN_CLOSE>
    {
        pub(super) fn new(writer: W) -> Self {
            Self {
                writer,
                started: false,
                initial_consumer: PhantomData,
            }
        }
    }

    #[cfg(todo)]
    impl<W: ConsumeTextChunk, InitialConsumer, S: ?Sized + HasConstState, const OPEN_CLOSE: u8>
        ConsumeOpenContentBeforeContent<K>
        for TConsumeOpenContentComma<W, InitialConsumer, S, OPEN_CLOSE>
    {
        type InitialConsumer = InitialConsumer;

        fn extend(&mut self, content: impl IntoJson<JsonKind = K>) {
            if self.started {
                let Consumed { .. } = content.json_provide_into(TConsumeContentComma(
                    self.writer.as_mut_consume_text_chunk(),
                ));
            } else {
                let Consumed { .. } = content.json_provide_into(TConsumeOpenItemsIfNotEmpty::new(
                    self.writer.as_mut_consume_text_chunk(),
                    &mut self.started,
                ));
            }
        }

        type End<const PREV_STATE: u128, const NEXT_STATE: u128> =
            TConsumeChunksOfNonEmpty<W, InitialConsumer, NextStateOf<S>, OPEN_CLOSE>;

        fn end<const PREV_STATE: u128, const NEXT_STATE: u128>(
            mut self,
            v: crate::r#const::IntermediateChunkAsStr<'_, PREV_STATE, NEXT_STATE>,
        ) -> Self::End<PREV_STATE, NEXT_STATE> {
            if self.started {
                self.writer.consume_text_chunk(v.as_str());
            } else {
                self.writer.consume_2_text_chunks(OPEN, v.as_str());
            }

            TConsumeChunksOfNonEmpty(self.writer, PhantomData)
        }
    }
});
