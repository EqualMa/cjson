use core::marker::PhantomData;

use crate::{
    ser::{
        IntoJson,
        traits::{ConsumeTextChunk, IntoTextChunks},
    },
    utils::impl_many,
};

use super::{
    ConsumeChained, ConsumeChunksOfNonEmptyArray, ConsumeChunksOfNonEmptyObject, ConsumeJson,
    ConsumeJsonText, Consumed,
    consume_chained_content::{ConsumeChainedArrayItems, ConsumeChainedObjectKvs},
    json_kinds::{self, JsonKindContains},
    json_string_chunks,
    open_close::OpenClose,
    states,
};

pub struct ConsumeStringFragment<W>(pub W);

/// TODO: is this needed?
pub struct ConsumeArrayItems<W>(pub W);
pub struct ConsumeObjectKvs<W>(pub W);

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

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeStringFragment<W> {
        type ConsumeJsonKind = json_kinds::JsonString;
        type Writer = W;

        not_any_value! {}
        not_array! {}
        not_object! {}

        fn consume_empty_string(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            only_expr!(Consumed::ASSERT_STRING)
        }
        fn consume_json_string_as_str(
            mut self,
            v: crate::r#const::JsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.fragment()));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        fn consume_str(
            mut self,
            s: &str,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![Consumed<json_kinds::JsonString, Self>] {
            de_async_move!(async move {
                () = await_try!(
                    crate::ser::texts::StrToJsonStringFragment(s).x_write_into(&mut self.0)
                );
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        type EndJsonString = json_string_chunks::EndJsonStringWithNothing;
        fn start_to_consume_chunks_of_json_string_with_first_chunk(
            mut self,
            v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.fragment()));
                last_expr!(CONSUME_IN_JSON_STRING::new(self.0))
            })
        }
        fn start_to_consume_chunks_of_json_string(
            mut self,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>] {
            de_async_move!(async move {
                let Consumed { .. } = await_try!(v.json_provide_into_x(ConsumeStringFragment(
                    self.0.as_mut_x_consume_text_chunk(),
                )));
                last_expr!(CONSUME_IN_JSON_STRING::new(self.0))
            })
        }

        type ConsumeChainedStrings = Self;

        fn start_to_consume_chained_strings(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::JsonString>,
        ) -> Self::ConsumeChainedStrings {
            self
        }
    }

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<json_kinds::JsonString> for ConsumeStringFragment<W> {
        fn extend<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            &mut self,
            s: V,
        ) -> Output![(), <Self::InitialConsumer as CONSUME_JSON>::Writer] {
            s.json_provide_into_x(ConsumeStringFragment(self.0.as_mut_x_consume_text_chunk()))
                .x_map_ok(|Consumed { .. }| ())
        }

        type InitialConsumer = Self;
        fn end_with<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            self,
            s: V,
        ) -> Output![
            Consumed<json_kinds::JsonString, Self::InitialConsumer>,
            <Self::InitialConsumer as CONSUME_JSON>::Writer
        ] {
            s.json_provide_into_x(self)
        }
    }

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeArrayItems<W> {
        type ConsumeJsonKind = json_kinds::Array;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_object! {}

        fn consume_empty_array(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            only_expr!(Consumed::ASSERT_ARRAY)
        }
        fn consume_non_empty_array_as_str(
            mut self,
            v: crate::r#const::NonEmptyArrayAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.items()));
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
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChunksOfNonEmptyArray {
            ConsumeChunksOfNonEmptyArray(self.0, PhantomData)
        }

        type ConsumeChainedArrays = ConsumeChainedArrayItems<W, bool, Self>;
        fn start_to_consume_chained_arrays(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Self::ConsumeChainedArrays {
            ConsumeChainedArrayItems::new_owned(self.0)
        }

        fn consume_array_of_items(
            mut self,
            items: impl IntoIterator<Item: IntoJson>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Array>,
        ) -> Output![Consumed<json_kinds::Array, Self>] {
            de_async_move!(async move {
                let mut items = items.into_iter();
                let Some(first) = items.next() else {
                    return last_expr!(Consumed::ASSERT_ARRAY);
                };
                let Consumed { .. } = await_try!(
                    first
                        .json_provide_into_x(ConsumeJsonText(self.0.as_mut_x_consume_text_chunk()))
                );
                () = await_try!(items.x_into_for_each(de_async!(async move |item| {
                    () = await_try!(self.0.x_consume_text_chunk(","));
                    let Consumed { .. } = await_try!(item.json_provide_into_x(ConsumeJsonText(
                        self.0.as_mut_x_consume_text_chunk()
                    )));
                    last_expr!(())
                })));

                last_expr!(Consumed::ASSERT_ARRAY)
            })
        }
    }

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeObjectKvs<W> {
        type ConsumeJsonKind = json_kinds::Object;
        type Writer = W;

        not_any_value! {}
        not_string! {}
        not_array! {}

        fn consume_empty_object(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            only_expr!(Consumed::ASSERT_OBJECT)
        }
        fn consume_non_empty_object_as_str(
            mut self,
            v: crate::r#const::NonEmptyObjectAsStr<'_>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.kvs()));
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
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChunksOfNonEmptyObject {
            ConsumeChunksOfNonEmptyObject(self.0, PhantomData)
        }

        type ConsumeChainedObjects = ConsumeChainedObjectKvs<W, bool, Self>;
        fn start_to_consume_chained_objects(
            self,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Self::ConsumeChainedObjects {
            ConsumeChainedObjectKvs::new_owned(self.0)
        }

        fn consume_object_of_iter(
            mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
            (): <Self::ConsumeJsonKind as JsonKindContains>::Contains<json_kinds::Object>,
        ) -> Output![Consumed<json_kinds::Object, Self>] {
            de_async_move!(async move {
                () = await_try!(self.0.x_write_kvs(kvs));
                last_expr!(Consumed::ASSERT_OBJECT)
            })
        }
    }
});
