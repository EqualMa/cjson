use core::marker::PhantomData;

use crate::{
    r#const::{HasConstState, states},
    ser::IntoJson,
    utils::impl_many,
};

use super::{
    ConsumeArrayItemsAppendCommaIfNotEmpty, ConsumeChunksOfNonEmptyArray,
    ConsumeChunksOfNonEmptyObject, ConsumeJsonText, Consumed, OpenClose, json_kinds,
    open_close::GroupOrComma,
};

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON_CHUNKS, CONSUME_JSON_CHUNKS_FROM_INIT, Output};
    }

    mod ready_array {
        pub trait ReadyToConsumeJsonChunksOfNonEmptyArray:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Array>
        {
        }
    }

    mod ready_try_array {
        pub trait ReadyToTryConsumeJsonChunksOfNonEmptyArray:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Array>
        {
        }

        use trait_mod::CONSUME_JSON;
    }

    mod ready_async_try_array {
        pub trait ReadyToAsyncTryConsumeJsonChunksOfNonEmptyArray:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Array>
        {
        }

        use trait_mod::CONSUME_JSON;
    }

    type LeftBracketValue: CONSUME_JSON_CHUNKS<
            json_kinds::Array,
            CurrentState = states::LeftBracketValue,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_bracket_value<V: IntoJson>(
        self,
        value: V,
    ) -> Output![
        Self::LeftBracketValue,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];

    type LeftBracketItemsBeforeItem: CONSUME_JSON_CHUNKS<
            json_kinds::Array,
            CurrentState = states::LeftBracketItemsBeforeItem,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_bracket_items_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
        self,
        items: V,
    ) -> Output![
        Self::LeftBracketItemsBeforeItem,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];
});

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_JSON_CHUNKS, CONSUME_JSON_CHUNKS_FROM_INIT, Output};
    }

    mod ready_object {
        pub trait ReadyToConsumeJsonChunksOfNonEmptyObject:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Object>
        {
        }
    }

    mod ready_try_object {
        pub trait ReadyToTryConsumeJsonChunksOfNonEmptyObject:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Object>
        {
        }

        use trait_mod::CONSUME_JSON;
    }

    mod ready_async_try_object {
        pub trait ReadyToAsyncTryConsumeJsonChunksOfNonEmptyObject:
            CONSUME_JSON_CHUNKS_FROM_INIT<json_kinds::Object>
        {
        }

        use trait_mod::CONSUME_JSON;
    }

    type LeftBraceKvsBeforeKv: CONSUME_JSON_CHUNKS<
            json_kinds::Object,
            CurrentState = states::LeftBraceKvsBeforeKv,
            InitialConsumer = Self::InitialConsumer,
        >;
    fn left_brace_kvs_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
        self,
        kvs: V,
    ) -> Output![
        Self::LeftBraceKvsBeforeKv,
        <Self::InitialConsumer as CONSUME_JSON>::Writer
    ];
});

impl_many!({
    {
        {
            use crate::ser::consumers::define_traits::base as trait_mod;

            use ImplEndWithRight as IMPL_END_WITH_RIGHT;
        }
        {
            use crate::ser::consumers::define_traits::try_ as trait_mod;

            use ImplTryEndWithRight as IMPL_END_WITH_RIGHT;
        }
        {
            use crate::ser::consumers::define_traits::async_try as trait_mod;

            use ImplAsyncTryEndWithRight as IMPL_END_WITH_RIGHT;
        }
    }

    use trait_mod::{
        CONSUME_JSON, CONSUME_TEXT_CHUNK, Output, READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY,
        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT, XHelpers as _, await_try, de_async_move,
        last_expr, never_future,
    };

    impl<W: CONSUME_TEXT_CHUNK, InitialConsumer: CONSUME_JSON<Writer = W>, const OC: u8>
        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_ARRAY
        for ConsumeChunksOfNonEmptyArray<W, InitialConsumer, states::Init, OC>
    {
        type LeftBracketValue =
            ConsumeChunksOfNonEmptyArray<W, InitialConsumer, states::LeftBracketValue, OC>;

        fn left_bracket_value<V: IntoJson>(
            mut self,
            value: V,
        ) -> Output![Self::LeftBracketValue, W] {
            de_async_move!(async move {
                match const { OpenClose::try_from_u8(OC).unwrap().open } {
                    GroupOrComma::Nothing => {}
                    GroupOrComma::Group => {
                        () = await_try!(self.0.x_consume_text_chunk("["));
                    }
                    GroupOrComma::Comma => {
                        () = await_try!(self.0.x_consume_text_chunk(","));
                    }
                }

                let Consumed { .. } = await_try!(
                    value
                        .json_provide_into_x(ConsumeJsonText(self.0.as_mut_x_consume_text_chunk()))
                );

                last_expr!(ConsumeChunksOfNonEmptyArray(self.0, PhantomData))
            })
        }

        type LeftBracketItemsBeforeItem = ConsumeChunksOfNonEmptyArray<
            W,
            InitialConsumer,
            states::LeftBracketItemsBeforeItem,
            OC,
        >;

        fn left_bracket_items_before_item<V: IntoJson<JsonKind = json_kinds::Array>>(
            mut self,
            items: V,
        ) -> Output![Self::LeftBracketItemsBeforeItem, W] {
            de_async_move!(async move {
                match const { OpenClose::try_from_u8(OC).unwrap().open } {
                    GroupOrComma::Nothing => {}
                    GroupOrComma::Group => {
                        () = await_try!(self.0.x_consume_text_chunk("["));
                    }
                    GroupOrComma::Comma => {
                        () = await_try!(self.0.x_consume_text_chunk(","));
                    }
                }

                let Consumed { .. } = await_try!(items.json_provide_into_x(
                    ConsumeArrayItemsAppendCommaIfNotEmpty(self.0.as_mut_x_consume_text_chunk(),)
                ));

                last_expr!(ConsumeChunksOfNonEmptyArray(self.0, PhantomData))
            })
        }
    }

    impl<W: CONSUME_TEXT_CHUNK, InitialConsumer: CONSUME_JSON<Writer = W>, const OC: u8>
        READY_TO_CONSUME_CHUNKS_OF_NON_EMPTY_OBJECT
        for ConsumeChunksOfNonEmptyObject<W, InitialConsumer, states::Init, OC>
    {
        type LeftBraceKvsBeforeKv =
            ConsumeChunksOfNonEmptyObject<W, InitialConsumer, states::LeftBraceKvsBeforeKv, OC>;
        fn left_brace_kvs_before_kv<V: IntoJson<JsonKind = json_kinds::Object>>(
            mut self,
            kvs: V,
        ) -> Output![Self::LeftBraceKvsBeforeKv, W] {
            de_async_move!(async move {
                match const { OpenClose::try_from_u8(OC).unwrap().open } {
                    GroupOrComma::Nothing => {}
                    GroupOrComma::Group => {
                        () = await_try!(self.0.x_consume_text_chunk("{"));
                    }
                    GroupOrComma::Comma => {
                        () = await_try!(self.0.x_consume_text_chunk(","));
                    }
                }
                let Consumed { .. } = await_try!(kvs.json_provide_into_x(
                    super::ConsumeObjectKvsAppendCommaIfNotEmpty(
                        self.0.as_mut_x_consume_text_chunk(),
                    )
                ));

                last_expr!(ConsumeChunksOfNonEmptyObject(self.0, PhantomData))
            })
        }
    }

    impl<W: CONSUME_TEXT_CHUNK, InitialConsumer, S: ?Sized + HasConstState, const OC: u8>
        IMPL_END_WITH_RIGHT<ConsumeChunksOfNonEmptyArray<W, InitialConsumer, S, OC>>
    {
        pub(crate) fn impl_end_with_right_bracket(
            self,
            (): (),
        ) -> Output![Consumed<json_kinds::Array, InitialConsumer>, W] {
            const { S::STATE.right_bracket().assert_eof_of_non_empty_array() }
            de_async_move!(async move {
                let Self(mut this) = self;
                match const { OpenClose::try_from_u8(OC).unwrap().close } {
                    GroupOrComma::Nothing => {}
                    GroupOrComma::Group => () = await_try!(this.0.x_consume_text_chunk("]")),
                    GroupOrComma::Comma => () = await_try!(this.0.x_consume_text_chunk(",")),
                }
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
        pub(crate) fn impl_end_with_right_brace(
            self,
            yes: core::convert::Infallible,
        ) -> Output![Consumed<json_kinds::Array, InitialConsumer>, W] {
            never_future!(match yes {})
        }
    }

    impl<W: CONSUME_TEXT_CHUNK, InitialConsumer, S: ?Sized + HasConstState, const OC: u8>
        IMPL_END_WITH_RIGHT<ConsumeChunksOfNonEmptyObject<W, InitialConsumer, S, OC>>
    {
        pub(crate) fn impl_end_with_right_bracket(
            self,
            yes: core::convert::Infallible,
        ) -> Output![Consumed<json_kinds::Object, InitialConsumer>, W] {
            never_future!(match yes {})
        }
        pub(crate) fn impl_end_with_right_brace(
            self,
            (): (),
        ) -> Output![Consumed<json_kinds::Object, InitialConsumer>, W] {
            const { S::STATE.right_brace().assert_eof_of_non_empty_object() }
            de_async_move!(async move {
                let Self(mut this) = self;
                match const { OpenClose::try_from_u8(OC).unwrap().close } {
                    GroupOrComma::Nothing => {}
                    GroupOrComma::Group => () = await_try!(this.0.x_consume_text_chunk("}")),
                    GroupOrComma::Comma => () = await_try!(this.0.x_consume_text_chunk(",")),
                }
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
    }
});

pub(crate) struct ImplEndWithRight<T>(pub(crate) T);
pub(crate) struct ImplTryEndWithRight<T>(pub(crate) T);
pub(crate) struct ImplAsyncTryEndWithRight<T>(pub(crate) T);
