use core::marker::PhantomData;

use crate::{
    r#const::states,
    ser::{IntoJson, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{
    ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject, ConsumeJson, Consumed,
    json_kinds::{self, JsonKind},
    open_close::OpenClose,
};

/// `$(, $item)* ]`
pub(super) struct ConsumeArrayCommaItemsClose<W>(pub W);
pub(super) struct ConsumeObjectCommaKvsClose<W>(pub W);

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
        CONSUME_CHAINED, CONSUME_IN_JSON_STRING, CONSUME_JSON, CONSUME_TEXT_CHUNK, Output,
        XHelpers as _, async_move_block, await_, await_try, de_async, de_async_move, last_expr,
        only_expr, select_method,
    };

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeArrayCommaItemsClose<W> {
        type ConsumeJsonKind = json_kinds::Array;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_object! {}

        fn consume_empty_array(
            mut self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk("]"));
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
        fn consume_non_empty_array_as_str(
            mut self,
            v: crate::r#const::NonEmptyArrayAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_2_text_chunks(",", v.items_close()));
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }

        type ConsumeChunksOfNonEmptyArray = ConsumeChunksOfNonEmptyArray<
            W,
            Self,
            states::Init,
            { OpenClose::PREPEND_COMMA_CLOSE_GROUP.as_u8() },
        >;
        fn start_to_consume_chunks_of_non_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
        }

        type ConsumeChainedArrays = Self;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            self
        }

        fn consume_array_of_items(
            mut self,
            items: impl IntoIterator<Item: IntoJson>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            de_async_move!(async move {
                let Consumed { .. } = await_try!(
                    super::ConsumeArrayItemsPrependCommaIfNotEmpty(
                        self.0.as_mut_x_consume_text_chunk(),
                    )
                    .consume_array_of_items(items, ())
                );

                () = await_try!(self.0.x_consume_text_chunk("]"));

                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
    }

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeObjectCommaKvsClose<W> {
        type ConsumeJsonKind = json_kinds::Object;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_array! {}

        fn consume_empty_object(
            mut self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk("}"));
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
        fn consume_non_empty_object_as_str(
            mut self,
            v: crate::r#const::NonEmptyObjectAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_2_text_chunks(",", v.kvs_close()));
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }

        type ConsumeChunksOfNonEmptyObject = ConsumeChunksOfNonEmptyObject<
            W,
            Self,
            states::Init,
            { OpenClose::PREPEND_COMMA_CLOSE_GROUP.as_u8() },
        >;
        fn start_to_consume_chunks_of_non_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
        }

        type ConsumeChainedObjects = Self;
        fn start_to_consume_chained_objects(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChainedObjects {
            self
        }

        fn consume_object_of_iter(
            mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            de_async_move!(async move {
                let Consumed { .. } = await_try!(
                    super::ConsumeObjectKvsPrependCommaIfNotEmpty(
                        self.0.as_mut_x_consume_text_chunk(),
                    )
                    .consume_object_of_iter(kvs, ())
                );

                () = await_try!(self.0.x_consume_text_chunk("}"));

                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
    }

    impl_many!({
        {
            {
                use super::ConsumeArrayItemsPrependCommaIfNotEmpty as ConsumeCommaContent;
                use ConsumeArrayCommaItemsClose as ConsumeCommaContentClose;
                use json_kinds::Array as K;
            }
            {
                use super::ConsumeObjectKvsPrependCommaIfNotEmpty as ConsumeCommaContent;
                use ConsumeObjectCommaKvsClose as ConsumeCommaContentClose;
                use json_kinds::Object as K;
            }
        }

        impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<K> for ConsumeCommaContentClose<W> {
            fn extend<V: IntoJson<JsonKind = K>>(&mut self, arr: V) -> Output![(), W] {
                arr.json_provide_into_x(ConsumeCommaContent(self.0.as_mut_x_consume_text_chunk()))
                    .x_map_ok(|Consumed { .. }| ())
            }

            type InitialConsumer = Self; // TODO:
            fn end_with<V: IntoJson<JsonKind = K>>(
                self,
                content: V,
            ) -> Output![Consumed<K, Self::InitialConsumer>, W] {
                // TODO: infinite recursion?
                content.json_provide_into_x(self)
            }
        }
    });
});
