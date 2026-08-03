use crate::utils::impl_many;

use super::{Consumed, consume_content::ConsumeStringFragment, json_kinds, json_string_chunks};

pub(super) struct ConsumeStringFragmentClose<W>(pub W);

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
        XHelpers as _, await_try, de_async_move, last_expr,
    };

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_JSON for ConsumeStringFragmentClose<W> {
        type ConsumeJsonKind = json_kinds::JsonString;
        type Writer = W;

        not_any_value! {}
        not_array! {}
        not_object! {}

        fn consume_empty_string(
            mut self,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Output![Consumed<json_kinds::JsonString, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk("\""));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        fn consume_str(
            mut self,
            s: &str,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Output![Consumed<json_kinds::JsonString, Self>, W] {
            de_async_move!(async move {
                () = await_try!(
                    crate::ser::texts::StrToJsonStringFragment(s).x_write_into(&mut self.0)
                );
                () = await_try!(self.0.x_consume_text_chunk("\""));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        fn consume_json_string_as_str(
            mut self,
            v: crate::r#const::JsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Output![Consumed<json_kinds::JsonString, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.fragment_close()));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }

        type EndJsonString = json_string_chunks::EndJsonStringWithClose;
        fn start_to_consume_chunks_of_json_string_with_first_chunk(
            mut self,
            v: crate::r#const::FirstChunkOfJsonStringAsStr<'_>,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>, W] {
            de_async_move!(async move {
                () = await_try!(self.0.x_consume_text_chunk(v.fragment()));
                let w = CONSUME_IN_JSON_STRING::new(self.0);
                last_expr!(w)
            })
        }
        fn start_to_consume_chunks_of_json_string(
            mut self,
            v: impl crate::ser::IntoJson<JsonKind = json_kinds::JsonString>,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Output![CONSUME_IN_JSON_STRING<Self::EndJsonString, Self>, W] {
            de_async_move!(async move {
                let Consumed { .. } = await_try!(v.json_provide_into_x(
                    super::consume_content::ConsumeStringFragment(
                        self.0.as_mut_x_consume_text_chunk(),
                    )
                ));
                let w = CONSUME_IN_JSON_STRING::new(self.0);
                last_expr!(w)
            })
        }

        type ConsumeChainedStrings = Self;
        fn start_to_consume_chained_strings(
            self,
            (): <Self::ConsumeJsonKind as json_kinds::JsonKindContains>::Contains<
                json_kinds::JsonString,
            >,
        ) -> Self::ConsumeChainedStrings {
            self
        }
    }

    impl<W: CONSUME_TEXT_CHUNK> CONSUME_CHAINED<json_kinds::JsonString>
        for ConsumeStringFragmentClose<W>
    {
        fn extend<V: crate::ser::IntoJson<JsonKind = json_kinds::JsonString>>(
            &mut self,
            s: V,
        ) -> Output![(), W] {
            s.json_provide_into_x(ConsumeStringFragment(self.0.as_mut_x_consume_text_chunk()))
                .x_map_ok(|Consumed { .. }| ())
        }

        type InitialConsumer = Self;
        fn end_with<V: crate::ser::IntoJson<JsonKind = json_kinds::JsonString>>(
            self,
            s: V,
        ) -> Output![Consumed<json_kinds::JsonString, Self::InitialConsumer>, W] {
            s.json_provide_into_x(self)
        }
    }
});
