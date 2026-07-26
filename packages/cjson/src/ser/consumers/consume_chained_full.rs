use crate::utils::impl_many;

use super::{
    ConsumeJsonText, Consumed, IntoJson, consume_content::ConsumeStringFragment,
    consume_content_close::ConsumeStringFragmentClose,
    consume_open_content::ConsumeStringOpenFragmentIfNotEmpty, json_kinds,
};

pub struct ConsumeChainedStringsFull<W> {
    writer: W,
    started: bool,
}

pub struct ConsumeChainedArraysFull<W> {
    writer: W,
    started: bool,
}

pub struct ConsumeChainedObjectsFull<W> {
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

    impl<W> ConsumeChainedFull<W> {
        pub(super) const fn new(writer: W) -> Self {
            Self {
                writer,
                started: false,
            }
        }
    }
});

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
        CONSUME_CHAINED, CONSUME_TEXT_CHUNK, Output, XHelpers as _, await_, await_try,
        de_async_move, last_expr,
    };

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<json_kinds::JsonString>
        for ConsumeChainedStringsFull<W>
    {
        fn extend<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            &mut self,
            s: V,
        ) -> Output![(), W] {
            de_async_move!(async move {
                if self.started {
                    let Consumed { .. } = await_try!(s.json_provide_into_x(ConsumeStringFragment(
                        self.writer.as_mut_x_consume_text_chunk(),
                    )));
                } else {
                    let Consumed { .. } = await_try!(s.json_provide_into_x(
                        ConsumeStringOpenFragmentIfNotEmpty::new(
                            self.writer.as_mut_x_consume_text_chunk(),
                            &mut self.started,
                        )
                    ));
                }

                last_expr!(())
            })
        }

        type InitialConsumer = ConsumeJsonText<W>;
        fn end_with<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            self,
            s: V,
        ) -> Output![Consumed<json_kinds::JsonString, Self::InitialConsumer>, W] {
            de_async_move!(async move {
                if self.started {
                    let Consumed { .. } =
                        await_try!(s.json_provide_into_x(ConsumeStringFragmentClose(self.writer)));
                    last_expr!(Consumed::ASSERT_STRING)
                } else {
                    await_!(s.json_provide_into_x(ConsumeJsonText(self.writer)))
                }
            })
        }
    }

    impl_many!({
        {
            {
                use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
                use super::consume_comma_content_close::ConsumeArrayCommaItemsClose as ConsumeCommaContentClose;
                use super::consume_open_content::ConsumeArrayOpenItemsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
                use ConsumeChainedArraysFull as ConsumeChainedFull;
                use json_kinds::Array as K;
            }
            {
                use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
                use super::consume_comma_content_close::ConsumeObjectCommaKvsClose as ConsumeCommaContentClose;
                use super::consume_open_content::ConsumeObjectOpenKvsIfNotEmpty as ConsumeOpenContentIfNotEmpty;
                use ConsumeChainedObjectsFull as ConsumeChainedFull;
                use json_kinds::Object as K;
            }
        }

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<K> for ConsumeChainedFull<W> {
            fn extend<V: IntoJson<JsonKind = K>>(&mut self, arr: V) -> Output![(), W] {
                de_async_move!(async move {
                    if self.started {
                        let Consumed { .. } = await_try!(arr.json_provide_into_x(
                            ConsumeCommaContent(self.writer.as_mut_x_consume_text_chunk(),)
                        ));
                    } else {
                        let Consumed { .. } =
                            await_try!(arr.json_provide_into_x(ConsumeOpenContentIfNotEmpty::new(
                                self.writer.as_mut_x_consume_text_chunk(),
                                &mut self.started,
                            )));
                    }

                    last_expr!(())
                })
            }

            type InitialConsumer = ConsumeJsonText<W>;
            fn end_with<V: IntoJson<JsonKind = K>>(
                self,
                arr: V,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                de_async_move!(async move {
                    if self.started {
                        let Consumed { .. } = await_try!(
                            arr.json_provide_into_x(ConsumeCommaContentClose(self.writer))
                        );
                        const { last_expr!(Consumed::assert(K)) }
                    } else {
                        await_!(arr.json_provide_into_x(ConsumeJsonText(self.writer)))
                    }
                })
            }
        }
    });
});
