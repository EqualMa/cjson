use core::{borrow::BorrowMut, marker::PhantomData};

use crate::utils::impl_many;

use super::{
    Consumed, IntoJson,
    consume_content::{ConsumeArrayItems, ConsumeObjectKvs},
    consume_content_and_record::{ConsumeArrayItemsAndRecord, ConsumeObjectKvsAndRecord},
    json_kinds,
};

pub struct ConsumeChainedArrayItems<
    W,
    S: BorrowMut<bool>,
    I: ConsumeChainedArrayItemsInitialConsumer,
> {
    writer: W,
    started: S,
    initial_consumer: PhantomData<I>,
}

pub struct ConsumeChainedObjectKvs<
    //
    W,
    S: BorrowMut<bool>,
    I: ConsumeChainedObjectKvsInitialConsumer,
> {
    writer: W,
    started: S,
    initial_consumer: PhantomData<I>,
}

pub trait ConsumeChainedArrayItemsInitialConsumer {}

impl<W> ConsumeChainedArrayItemsInitialConsumer for ConsumeArrayItems<W> {}
impl<W> ConsumeChainedArrayItemsInitialConsumer for ConsumeArrayItemsAndRecord<'_, W> {}

pub trait ConsumeChainedObjectKvsInitialConsumer {}

impl<W> ConsumeChainedObjectKvsInitialConsumer for ConsumeObjectKvs<W> {}
impl<W> ConsumeChainedObjectKvsInitialConsumer for ConsumeObjectKvsAndRecord<'_, W> {}

impl_many!({
    {
        {
            use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::consume_content_and_record::ConsumeArrayItemsAndRecord as ConsumeContentAndRecord;
            use ConsumeChainedArrayItems as ConsumeChainedContent;
            use ConsumeChainedArrayItemsInitialConsumer as TraitInitialConsumer;
            use json_kinds::Array as K;
        }
        {
            use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
            use super::consume_content_and_record::ConsumeObjectKvsAndRecord as ConsumeContentAndRecord;
            use ConsumeChainedObjectKvs as ConsumeChainedContent;
            use ConsumeChainedObjectKvsInitialConsumer as TraitInitialConsumer;
            use json_kinds::Object as K;
        }
    }

    impl<W, I: TraitInitialConsumer> ConsumeChainedContent<W, bool, I> {
        pub(super) fn new_owned(writer: W) -> Self {
            Self {
                writer,
                started: false,
                initial_consumer: PhantomData,
            }
        }
    }

    impl<W, S: BorrowMut<bool>, I: TraitInitialConsumer> ConsumeChainedContent<W, S, I> {
        pub(super) fn new(writer: W, mut started: S) -> Self {
            debug_assert!(!*started.borrow_mut());
            Self {
                writer,
                started,
                initial_consumer: PhantomData,
            }
        }
    }

    impl_many!({
        {
            {
                use crate::ser::consumers::define_traits::base as trait_mod;
            }
            {
                use crate::ser::consumers::define_traits::try_ as trait_mod;
            }
            {
                use crate::ser::consumers::define_traits::async_try as trait_mod;
            }
        }

        use trait_mod::{
            AwaitedOutput, CONSUME_CHAINED, CONSUME_JSON, CONSUME_TEXT_CHUNK, Output,
            XHelpers as _, await_try, de_async, last_expr,
        };

        struct ImplExtend<T>(T);

        impl<W: CONSUME_TEXT_CHUNK, S: BorrowMut<bool>, I: TraitInitialConsumer>
            ImplExtend<ConsumeChainedContent<W, S, I>>
        {
            de_async!(
                async fn impl_extend(
                    self,
                    arr: impl IntoJson<JsonKind = K>,
                ) -> AwaitedOutput![(), W] {
                    let Self(mut this) = self;
                    let started = this.started.borrow_mut();
                    if *started {
                        let Consumed { .. } =
                            await_try!(arr.json_provide_into_x(ConsumeCommaContent(this.writer)));
                    } else {
                        let Consumed { .. } = await_try!(arr.json_provide_into_x(
                            ConsumeContentAndRecord::new(started, this.writer)
                        ));
                    }

                    last_expr!(())
                }
            );
        }

        impl<
            W: CONSUME_TEXT_CHUNK,
            S: BorrowMut<bool>,
            I: TraitInitialConsumer + CONSUME_JSON<Writer = W>,
        > CONSUME_CHAINED<K> for ConsumeChainedContent<W, S, I>
        {
            fn extend<V: IntoJson<JsonKind = K>>(&mut self, arr: V) -> Output![(), W] {
                ImplExtend(ConsumeChainedContent {
                    writer: self.writer.as_mut_x_consume_text_chunk(),
                    started: self.started.borrow_mut(),
                    initial_consumer: PhantomData::<I>,
                })
                .impl_extend(arr)
            }

            type InitialConsumer = I;
            fn end_with<V: IntoJson<JsonKind = K>>(
                self,
                arr: V,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                ImplExtend(self)
                    .impl_extend(arr)
                    .x_map_ok(|()| const { Consumed::assert(K) })
            }
        }
    });
});
