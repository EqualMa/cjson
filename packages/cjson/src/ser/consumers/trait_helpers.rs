macro_rules! define_one {
    (
        {
            $vis:vis $mod:ident $mod_name:ident { $($mod_items:item)* }
        }{
            $($common_items:item)*
        }
    ) => {
        $vis $mod $mod_name {
            use crate::ser::consumers::define_traits::$mod_name as trait_mod;
            $($mod_items)*
            $($common_items)*
        }
    };
}

macro_rules! define_many {
    (
        { $($vis:vis $mod:ident $mod_name:ident $mod_items_braced:tt)* }
        $common_items_braced:tt
    ) => {
        $(define_one! {
            { $vis $mod $mod_name $mod_items_braced }
            $common_items_braced
        })*
    };
}

macro_rules! define {
    ({
        $mods_braced:tt

        $($common_items:item)*
    }) => {
        define_many! {
            $mods_braced
            {$($common_items)*}
        }
    };
}

pub(crate) trait _Result: Sized {
    type _Ok;
    type _Err;
    fn _map<U>(self, f: impl FnOnce(Self::_Ok) -> U) -> Result<U, Self::_Err>;
}

impl<T, E> _Result for Result<T, E> {
    type _Ok = T;
    type _Err = E;
    #[inline]
    fn _map<U>(self, f: impl FnOnce(Self::_Ok) -> U) -> Result<U, Self::_Err> {
        self.map(f)
    }
}

pub(crate) trait _TryFuture: Future<Output = Result<Self::_Ok, Self::_Err>> {
    type _Ok;
    type _Err;
    fn _map_ok<U>(
        self,
        f: impl FnOnce(Self::_Ok) -> U,
    ) -> impl Future<Output = Result<U, Self::_Err>>;
}

impl<F: Future<Output = Result<T, E>>, T, E> _TryFuture for F {
    type _Ok = T;
    type _Err = E;
    #[inline]
    fn _map_ok<U>(
        self,
        f: impl FnOnce(Self::_Ok) -> U,
    ) -> impl Future<Output = Result<U, Self::_Err>> {
        async { self.await.map(f) }
    }
}

define!({
    {
        pub(crate) mod base {
            use Sized as XMapAwaitedOk;
        }
        pub(crate) mod try_ {
            use super::_Result as XMapAwaitedOk;
        }
        pub(crate) mod async_try {
            use super::_TryFuture as XMapAwaitedOk;
        }
    }

    use crate::ser::{
        ConsumeJsonText, Consumed, IntoJson, consumers::consume_content, json_kinds::JsonKind,
    };

    use trait_mod::{
        CONSUME_JSON, CONSUME_TEXT_CHUNK, Output, await_, await_try, de_async, de_async_move,
        last_expr, select, select_expr, select_method, select_type,
    };

    pub(crate) trait XHelpers {
        select![
            {
                fn x_into_for_each(self, f: impl FnMut(Self::Item))
                where
                    Self: IntoIterator + Sized,
                {
                    self.into_iter().for_each(f)
                }
            },
            {
                fn x_into_for_each<E>(
                    self,
                    f: impl FnMut(Self::Item) -> Result<(), E>,
                ) -> Result<(), E>
                where
                    Self: IntoIterator + Sized,
                {
                    self.into_iter().try_for_each(f)
                }
            },
            {
                fn x_into_for_each<E>(
                    self,
                    mut f: impl AsyncFnMut(Self::Item) -> Result<(), E>,
                ) -> impl Future<Output = Result<(), E>>
                where
                    Self: IntoIterator + Sized,
                {
                    async move {
                        for item in self {
                            () = f(item).await?;
                        }

                        Ok(())
                    }
                }
            },
        ];

        fn x_write_into<W: ?Sized + CONSUME_TEXT_CHUNK>(self, w: &mut W) -> Output![(), W]
        where
            Self: crate::ser::traits::IntoTextChunks + Sized,
        {
            select_method!(
                self.write_into(w)
                    //
                    .try_write_into
                    .async_try_write_into
            )
        }

        fn x_consume_text_chunk(&mut self, chunk: &str) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK,
        {
            select_method!(
                self.consume_text_chunk(chunk)
                    .try_consume_text_chunk
                    .async_try_consume_text_chunk
            )
        }

        fn x_consume_2_text_chunks(&mut self, chunk1: &str, chunk2: &str) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK,
        {
            select_method!(
                self.consume_2_text_chunks(chunk1, chunk2)
                    .try_consume_2_text_chunks
                    .async_try_consume_2_text_chunks
            )
        }

        fn as_mut_x_consume_text_chunk(
            &mut self,
        ) -> select! {
            { impl CONSUME_TEXT_CHUNK                  },
            { impl CONSUME_TEXT_CHUNK<Err = Self::Err> },
            { impl CONSUME_TEXT_CHUNK<Err = Self::Err> },
        }
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            select_method!(
                self.as_mut_consume_text_chunk()
                    .as_mut_try_consume_text_chunk
                    .as_mut_async_try_consume_text_chunk
            )
        }

        fn json_provide_into_x<
            W: CONSUME_JSON<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
        >(
            self,
            w: W,
        ) -> Output![Consumed<Self::JsonKind, W>, W::Writer]
        where
            Self: Sized + IntoJson,
        {
            select_method!(
                self.json_provide_into(w)
                    .json_provide_into_try
                    .json_provide_into_async_try
            )
        }

        fn json_provide_to_x<
            W: CONSUME_JSON<ConsumeJsonKind: JsonKind<Contains<Self::ToJsonKind> = ()>>,
        >(
            &self,
            w: W,
        ) -> Output![Consumed<Self::ToJsonKind, W>, W::Writer]
        where
            Self: crate::ser::ToJson2,
        {
            select_method!(
                self.json_provide_to(w)
                    .json_provide_to_try
                    .json_provide_to_async_try
            )
        }

        fn x_write_key_frag_quote_colon_value(
            &mut self,
            kv: impl crate::ser::IntoJsonKeyColonValue,
        ) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            de_async_move!(async move {
                let (key, value) = kv.into_json_key_value();
                let Consumed { .. } = await_try!(key.json_provide_into_x(
                    consume_content::ConsumeStringFragment(self.as_mut_x_consume_text_chunk())
                ));
                () = await_try!(self.x_consume_text_chunk("\":"));
                let Consumed { .. } = await_try!(
                    value.json_provide_into_x(ConsumeJsonText(self.as_mut_x_consume_text_chunk()))
                );
                last_expr!(())
            })
        }

        fn x_write_comma_kvs(
            &mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        ) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            // ,"key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value
            kvs.x_into_for_each(de_async!(async move |kv| {
                () = await_try!(self.x_consume_text_chunk(",\""));
                () = await_try!(self.x_write_key_frag_quote_colon_value(kv));
                last_expr!(())
            }))
        }

        fn x_write_non_empty_kvs(
            &mut self,
            first: impl crate::ser::IntoJsonKeyColonValue,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        ) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            // "key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value
            de_async_move!(async move {
                () = await_try!(self.x_consume_text_chunk("\"")); // TODO: ConsumeStringOpenFragment
                () = await_try!(self.x_write_key_frag_quote_colon_value(first));

                () = await_try!(self.x_write_comma_kvs(kvs));

                last_expr!(())
            })
        }

        fn x_write_kvs(
            &mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        ) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            de_async_move!(async move {
                let mut kvs = kvs.into_iter();
                let Some(first) = kvs.next() else {
                    return last_expr!(());
                };

                await_!(self.x_write_non_empty_kvs(first, kvs))
            })
        }

        fn x_write_kvs_comma(
            &mut self,
            kvs: impl IntoIterator<Item: crate::ser::IntoJsonKeyColonValue>,
        ) -> Output![(), Self]
        where
            Self: CONSUME_TEXT_CHUNK + Sized,
        {
            // "key_frag_quote_colon_value,"key_frag_quote_colon_value,"key_frag_quote_colon_value,
            de_async_move!(async move {
                let mut kvs = kvs.into_iter();
                let Some(first) = kvs.next() else {
                    return last_expr!(());
                };

                () = await_try!(self.x_write_non_empty_kvs(first, kvs));

                await_!(self.x_consume_text_chunk(","))
            })
        }

        #[inline]
        fn x_map_ok<U>(
            self,
            f: select_type![
                impl FnOnce(Self) -> U,
                impl FnOnce(Self::_Ok) -> U,
                impl FnOnce(Self::_Ok) -> U,
            ],
        ) -> select_type![
            U,
            Result::<U, Self::_Err>,
            impl Future<Output = Result<U, Self::_Err>>,
        ]
        where
            Self: XMapAwaitedOk + Sized,
        {
            select_expr!(
                //
                f(self),
                self._map(f),
                self._map_ok(f),
            )
        }

        de_async!(
            async fn x_map_ref_1<U, A1>(
                self,
                arg: A1,
                f: select_type![
                    impl FnOnce(&Self, A1) -> U,
                    impl FnOnce(&Self, A1) -> U,
                    impl AsyncFnOnce(&Self, A1) -> U,
                ],
            ) -> U
            where
                Self: Sized,
            {
                await_!(f(&self, arg))
            }
        );
    }

    impl<T: ?Sized> XHelpers for T {}
});
