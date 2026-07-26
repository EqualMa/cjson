use core::marker::PhantomData;

use crate::{
    ser::{IntoJson, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{
    ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject, ConsumeJson, ConsumeJsonText,
    Consumed,
    consume_chained_content::{ConsumeChainedArrayItems, ConsumeChainedObjectKvs},
    json_kinds::{self, JsonKind},
    never_consume::NeverConsume,
    open_close::OpenClose,
    states,
};

pub(super) struct ConsumeArrayItemsAndRecord<'a, W> {
    /// should be initialized as false
    started: &'a mut bool,
    writer: W,
}

pub(super) struct ConsumeObjectKvsAndRecord<'a, W> {
    /// should be initialized as false
    started: &'a mut bool,
    writer: W,
}

impl_many!({
    {
        {
            use ConsumeArrayItemsAndRecord as ConsumeContentAndRecord;
        }
        {
            use ConsumeObjectKvsAndRecord as ConsumeContentAndRecord;
        }
    }

    impl<'a, W> ConsumeContentAndRecord<'a, W> {
        pub(super) const fn new(started: &'a mut bool, writer: W) -> Self {
            debug_assert!(!*started);
            Self { started, writer }
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
        CONSUME_CHAINED, CONSUME_IN_JSON_STRING, CONSUME_JSON, CONSUME_TEXT_CHUNK, Output,
        XHelpers as _, async_move_block, await_, await_try, de_async, de_async_move, last_expr,
        never_future, only_expr, select_method,
    };

    impl<'a, W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeArrayItemsAndRecord<'a, W> {
        type ConsumeJsonKind = json_kinds::Array;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_object! {}

        fn consume_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            debug_assert!(!*self.started);
            only_expr!(Consumed::ASSERT_ARRAY)
        }
        fn consume_non_empty_array_as_str(
            mut self,
            v: crate::r#const::NonEmptyArrayAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            debug_assert!(!*self.started);
            de_async_move!(async move {
                *self.started = true;
                () = await_try!(self.writer.x_consume_text_chunk(v.items()));
                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }

        type ConsumeChunksOfNonEmptyArray = ConsumeChunksOfNonEmptyArray<
            W,
            Self,
            states::Init,
            { OpenClose::BOTH_NOTHING.as_u8() },
        >;
        fn start_to_consume_chunks_of_non_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            debug_assert!(!*self.started);
            *self.started = true;
            ConsumeChunksOfNonEmptyArray(self.writer, PhantomData)
        }

        type ConsumeChainedArrays = ConsumeChainedArrayItems<W, &'a mut bool, Self>;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            debug_assert!(!*self.started);
            ConsumeChainedArrayItems::new(self.writer, self.started)
        }

        fn consume_array_of_items(
            mut self,
            items: impl IntoIterator<Item: crate::ser::IntoJson>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            debug_assert!(!*self.started);
            de_async_move!(async move {
                let mut items = items.into_iter();
                let Some(first) = items.next() else {
                    return last_expr!(Consumed::ASSERT_ARRAY);
                };
                *self.started = true;
                let Consumed { .. } = await_try!(first.json_provide_into_x(ConsumeJsonText(
                    self.writer.as_mut_x_consume_text_chunk()
                )));

                () = await_try!(items.x_into_for_each(de_async!(async move |item| {
                    () = await_try!(self.writer.x_consume_text_chunk(","));
                    let Consumed { .. } = await_try!(item.json_provide_into_x(ConsumeJsonText(
                        self.writer.as_mut_x_consume_text_chunk(),
                    )));
                    last_expr!(())
                })));

                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
    }

    impl<'a, W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeObjectKvsAndRecord<'a, W> {
        type ConsumeJsonKind = json_kinds::Object;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_array! {}

        fn consume_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            debug_assert!(!*self.started);
            only_expr!(Consumed::ASSERT_OBJECT)
        }
        fn consume_non_empty_object_as_str(
            mut self,
            v: crate::r#const::NonEmptyObjectAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            debug_assert!(!*self.started);
            de_async_move!(async move {
                *self.started = true;
                () = await_try!(self.writer.x_consume_text_chunk(v.kvs()));
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }

        type ConsumeChunksOfNonEmptyObject = ConsumeChunksOfNonEmptyObject<
            W,
            Self,
            states::Init,
            { OpenClose::BOTH_NOTHING.as_u8() },
        >;
        fn start_to_consume_chunks_of_non_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            debug_assert!(!*self.started);
            *self.started = true;
            ConsumeChunksOfNonEmptyObject(self.writer, PhantomData)
        }

        type ConsumeChainedObjects = ConsumeChainedObjectKvs<W, &'a mut bool, Self>;
        fn start_to_consume_chained_objects(
            self,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChainedObjects {
            debug_assert!(!*self.started);
            ConsumeChainedObjectKvs::new(self.writer, self.started)
        }

        fn consume_object_of_iter(
            mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            (): <Self::ConsumeJsonKind as JsonKind>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            debug_assert!(!*self.started);
            de_async_move!(async move {
                let mut kvs = kvs.into_iter();
                let Some(first) = kvs.next() else {
                    return last_expr!(Consumed::ASSERT_OBJECT);
                };
                *self.started = true;
                () = await_try!(self.writer.x_write_non_empty_kvs(first, kvs));
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
    }
});
