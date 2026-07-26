use crate::{
    r#const::{JsonStringFragmentAsStr, LastChunkOfJsonStringAsStr},
    ser::{IntoJson, json_kinds, traits::ConsumeTextChunk},
    utils::impl_many,
};

use super::{ConsumeJson, Consumed};

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_TEXT_CHUNK, Output};
    }
    mod consume_fragment_in_string {
        pub trait ConsumeFragmentInString {}
    }
    mod try_consume_fragment_in_string {
        pub trait TryConsumeFragmentInString {}
    }
    mod async_try_consume_fragment_in_string {
        pub trait AsyncTryConsumeFragmentInString {}
    }

    fn consume_fragment_as_str<W: CONSUME_TEXT_CHUNK>(
        &mut self,
        w: &mut W,
        v: JsonStringFragmentAsStr<'_>,
    ) -> Output![(), W];
    fn consume_fragment<W: CONSUME_TEXT_CHUNK>(
        &mut self,
        w: &mut W,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Output![(), W];
});

define_traits!({
    #[common_items]
    {
        use trait_mod::{CONSUME_TEXT_CHUNK, Output};
    }
    mod end_json_string {
        pub trait EndJsonString: ConsumeFragmentInString {}
    }
    mod try_end_json_string {
        pub trait TryEndJsonString: TryConsumeFragmentInString {}
    }
    mod async_try_end_json_string {
        pub trait AsyncTryEndJsonString: AsyncTryConsumeFragmentInString {}
    }

    fn end_with_last_chunk<W: CONSUME_TEXT_CHUNK>(
        //
        self,
        w: W,
        v: LastChunkOfJsonStringAsStr<'_>,
    ) -> Output![(), W];
    fn end_with<W: CONSUME_TEXT_CHUNK>(
        self,
        w: W,
        v: impl IntoJson<JsonKind = json_kinds::JsonString>,
    ) -> Output![(), W];
});

pub(crate) trait HasConstDefault {
    const DEFAULT: Self;
}

pub struct ConsumeInJsonString<E: EndJsonString, InitialConsumer: ?Sized + ConsumeJson> {
    end: E,
    writer: InitialConsumer::Writer,
}

pub struct TryConsumeInJsonString<
    E: TryEndJsonString,
    InitialConsumer: ?Sized + super::TryConsumeJson,
> {
    end: E,
    writer: InitialConsumer::Writer,
}

pub struct AsyncTryConsumeInJsonString<
    E: AsyncTryEndJsonString,
    InitialConsumer: ?Sized + super::AsyncTryConsumeJson,
> {
    end: E,
    writer: InitialConsumer::Writer,
}

pub trait ConsumeFragmentInStringNormally {}

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
        CONSUME_FRAGMENT_IN_STRING, CONSUME_IN_JSON_STRING, CONSUME_JSON, CONSUME_TEXT_CHUNK,
        END_JSON_STRING, Output, XHelpers as _, async_move_block, await_try, de_async_move,
        last_expr, never_future, select_method,
    };

    impl<T: ?Sized + ConsumeFragmentInStringNormally> CONSUME_FRAGMENT_IN_STRING for T {
        fn consume_fragment_as_str<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            w: &mut W,
            v: JsonStringFragmentAsStr<'_>,
        ) -> Output![(), W] {
            select_method!(
                w.consume_text_chunk(v.as_str())
                    .try_consume_text_chunk
                    .async_try_consume_text_chunk
            )
        }
        fn consume_fragment<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            w: &mut W,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                let Consumed { .. } = select_method!(
                    v.json_provide_into(super::consume_content::ConsumeStringFragment(
                        select_method!(
                            w.as_mut_consume_text_chunk()
                                .as_mut_try_consume_text_chunk
                                .as_mut_async_try_consume_text_chunk
                        )
                    ))
                    .json_provide_into_try
                    .json_provide_into_async_try
                    .await?
                );
                last_expr!(())
            })
        }
    }

    impl<E: END_JSON_STRING, InitialConsumer: ?Sized + CONSUME_JSON>
        CONSUME_IN_JSON_STRING<E, InitialConsumer>
    {
        pub(super) const fn new_full(end: E, writer: InitialConsumer::Writer) -> Self {
            Self { end, writer }
        }
        pub(super) const fn new(writer: InitialConsumer::Writer) -> Self
        where
            E: HasConstDefault,
        {
            Self::new_full(E::DEFAULT, writer)
        }
    }

    impl<E: END_JSON_STRING, InitialConsumer: ?Sized + CONSUME_JSON>
        CONSUME_IN_JSON_STRING<E, InitialConsumer>
    {
        pub fn consume_fragment_as_str(
            &mut self,
            v: JsonStringFragmentAsStr<'_>,
        ) -> Output![(), InitialConsumer::Writer] {
            self.end.consume_fragment_as_str(&mut self.writer, v)
        }
        pub fn consume_fragment<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            &mut self,
            v: V,
        ) -> Output![(), InitialConsumer::Writer] {
            self.end.consume_fragment(&mut self.writer, v)
        }

        pub fn end_with_last_chunk(
            self,
            v: LastChunkOfJsonStringAsStr<'_>,
        ) -> Output![
            Consumed<json_kinds::JsonString, InitialConsumer>,
            InitialConsumer::Writer
        ] {
            de_async_move!(async move {
                await_try!(self.end.end_with_last_chunk(self.writer, v));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }
        pub fn end_with<V: IntoJson<JsonKind = json_kinds::JsonString>>(
            self,
            v: V,
        ) -> Output![
            Consumed<json_kinds::JsonString, InitialConsumer>,
            InitialConsumer::Writer
        ] {
            de_async_move!(async move {
                await_try!(self.end.end_with(self.writer, v));
                last_expr!(Consumed::ASSERT_STRING)
            })
        }
    }

    impl CONSUME_FRAGMENT_IN_STRING for NeverEndJsonString {
        fn consume_fragment_as_str<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            _: &mut W,
            _: JsonStringFragmentAsStr<'_>,
        ) -> Output![(), W] {
            never_future!(match *self {})
        }

        fn consume_fragment<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            _: &mut W,
            _: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            never_future!(match *self {})
        }
    }

    impl END_JSON_STRING for NeverEndJsonString {
        fn end_with_last_chunk<W: CONSUME_TEXT_CHUNK>(
            //
            self,
            _: W,
            _: LastChunkOfJsonStringAsStr<'_>,
        ) -> Output![(), W] {
            never_future!(match self {})
        }

        fn end_with<W: CONSUME_TEXT_CHUNK>(
            self,
            _: W,
            _: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            never_future!(match self {})
        }
    }

    impl END_JSON_STRING for EndJsonStringWithClose {
        fn end_with_last_chunk<W: CONSUME_TEXT_CHUNK>(
            //
            self,
            mut w: W,
            v: LastChunkOfJsonStringAsStr<'_>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                select_method!(
                    w.consume_text_chunk(v.as_str())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await
                )
            })
        }

        fn end_with<W: CONSUME_TEXT_CHUNK>(
            self,
            w: W,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                let Consumed { .. } = await_try!(v.json_provide_into_x(
                    super::consume_content_close::ConsumeStringFragmentClose(w)
                ));
                last_expr!(())
            })
        }
    }

    impl END_JSON_STRING for EndJsonStringWithNothing {
        fn end_with_last_chunk<W: CONSUME_TEXT_CHUNK>(
            //
            self,
            mut w: W,
            v: LastChunkOfJsonStringAsStr<'_>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                select_method!(
                    w.consume_text_chunk(v.fragment())
                        .try_consume_text_chunk
                        .async_try_consume_text_chunk
                        .await
                )
            })
        }

        fn end_with<W: CONSUME_TEXT_CHUNK>(
            self,
            w: W,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                let Consumed { .. } = select_method!(
                    v.json_provide_into(super::consume_content::ConsumeStringFragment(w))
                        .json_provide_into_try
                        .json_provide_into_async_try
                        .await?
                );
                last_expr!(())
            })
        }
    }

    impl CONSUME_FRAGMENT_IN_STRING for EndJsonStringOpenFragmentIfNotEmpty<'_> {
        fn consume_fragment_as_str<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            w: &mut W,
            v: JsonStringFragmentAsStr<'_>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                let Some(non_empty_fragment) = v.non_empty_fragment() else {
                    return last_expr!(());
                };
                if *self.started {
                    select_method!(
                        w.consume_text_chunk(non_empty_fragment)
                            .try_consume_text_chunk
                            .async_try_consume_text_chunk
                            .await
                    )
                } else {
                    *self.started = true;
                    select_method!(
                        w.consume_2_text_chunks("\"", non_empty_fragment)
                            .try_consume_2_text_chunks
                            .async_try_consume_2_text_chunks
                            .await
                    )
                }
            })
        }

        fn consume_fragment<W: CONSUME_TEXT_CHUNK>(
            &mut self,
            w: &mut W,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            <_ as END_JSON_STRING>::end_with(
                EndJsonStringOpenFragmentIfNotEmpty {
                    started: self.started,
                },
                select_method!(
                    w.as_mut_consume_text_chunk()
                        .as_mut_try_consume_text_chunk
                        .as_mut_async_try_consume_text_chunk
                ),
                v,
            )
        }
    }
    impl<'a> END_JSON_STRING for EndJsonStringOpenFragmentIfNotEmpty<'a> {
        fn end_with_last_chunk<W: CONSUME_TEXT_CHUNK>(
            //
            self,
            mut w: W,
            v: LastChunkOfJsonStringAsStr<'_>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                let Some(non_empty_fragment) = v.non_empty_fragment() else {
                    return last_expr!(());
                };

                if *self.started {
                    select_method!(
                        w.consume_text_chunk(non_empty_fragment)
                            .try_consume_text_chunk
                            .async_try_consume_text_chunk
                            .await
                    )
                } else {
                    *self.started = true;
                    select_method!(
                        w.consume_2_text_chunks("\"", non_empty_fragment)
                            .try_consume_2_text_chunks
                            .async_try_consume_2_text_chunks
                            .await
                    )
                }
            })
        }

        fn end_with<W: CONSUME_TEXT_CHUNK>(
            self,
            w: W,
            v: impl IntoJson<JsonKind = json_kinds::JsonString>,
        ) -> Output![(), W] {
            de_async_move!(async move {
                if *self.started {
                    let Consumed { .. } = select_method!(
                        v.json_provide_into(super::consume_content::ConsumeStringFragment(w))
                            .json_provide_into_try
                            .json_provide_into_async_try
                            .await?
                    );
                } else {
                    let Consumed { .. } = select_method!(
                        v.json_provide_into(
                            super::consume_open_content::ConsumeStringOpenFragmentIfNotEmpty::new(
                                w,
                                self.started,
                            )
                        )
                        .json_provide_into_try
                        .json_provide_into_async_try
                        .await?
                    );
                }
                last_expr!(())
            })
        }
    }
});

pub enum NeverEndJsonString {}
pub struct EndJsonStringWithClose;
pub struct EndJsonStringWithNothing;

impl ConsumeFragmentInStringNormally for EndJsonStringWithClose {}

impl ConsumeFragmentInStringNormally for EndJsonStringWithNothing {}

impl HasConstDefault for EndJsonStringWithClose {
    const DEFAULT: Self = Self;
}

impl HasConstDefault for EndJsonStringWithNothing {
    const DEFAULT: Self = Self;
}

pub struct EndJsonStringOpenFragmentIfNotEmpty<'a> {
    pub(crate) started: &'a mut bool,
}
