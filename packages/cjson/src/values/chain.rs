// TODO: optimize IntoJson with IS_CHAINABLE_AND_ALWAYS_EMPTY
use crate::{
    ser::{ConsumeJson, IntoJson, ToJson2, helpers::json_fns, json_kinds, traits},
    utils::impl_many,
};

impl_many!({
    {
        {
            use crate::ser::json_kinds::Array as K;

            use super::ChainArray as Chain;

            #[inline]
            fn start_to_consume_chained<
                W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<K> = ()>>,
            >(
                w: W,
            ) -> W::ConsumeChainedArrays {
                w.start_to_consume_chained_arrays(())
            }

            macro_rules! chained_select_method {
                ($w:tt .$array:ident $args:tt .$object:ident .$string:ident) => {
                    $w.$array $args
                };
            }
        }
        {
            use crate::ser::json_kinds::Object as K;

            use super::ChainObject as Chain;

            #[inline]
            fn start_to_consume_chained<
                W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<K> = ()>>,
            >(
                w: W,
            ) -> W::ConsumeChainedObjects {
                w.start_to_consume_chained_objects(())
            }

            macro_rules! chained_select_method {
                ($w:tt .$array:ident $args:tt .$object:ident .$string:ident) => {
                    $w.$object $args
                };
            }
        }
        {
            use crate::ser::json_kinds::JsonString as K;

            use super::ChainString as Chain;

            #[inline]
            fn start_to_consume_chained<
                W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<K> = ()>>,
            >(
                w: W,
            ) -> W::ConsumeChainedStrings {
                w.start_to_consume_chained_strings(())
            }

            macro_rules! chained_select_method {
                ($w:tt .$array:ident $args:tt .$object:ident .$string:ident) => {
                    $w.$string $args
                };
            }
        }
    }

    impl<A: IntoJson<JsonKind = K>, B: IntoJson<JsonKind = K>> IntoJson for Chain<A, B> {
        type JsonKind = K;

        json_fns!({
            json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
            use trait_mod;
            async |self, w| {
                use trait_mod::{CONSUME_CHAINED as _, XHelpers as _, await_, await_try};
                if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                    await_!(self.1.json_provide_into_x(w))
                } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                    await_!(self.0.json_provide_into_x(w))
                } else {
                    let mut w = chained_select_method!(
                        w.start_to_consume_chained_arrays(())
                            .start_to_consume_chained_objects
                            .start_to_consume_chained_strings
                    );
                    () = await_try!(w.extend(self.0));
                    await_!(w.end_with(self.1))
                }
            }
        });

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }

    impl<A: ToJson2<ToJsonKind = K>, B: ToJson2<ToJsonKind = K>> ToJson2 for Chain<A, B> {
        type ToJsonKind = K;

        json_fns!({
            json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
            use trait_mod;
            async |&self, w| {
                use trait_mod::{CONSUME_CHAINED as _, XHelpers as _, await_, await_try};
                if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                    await_!(self.1.json_provide_to_x(w))
                } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                    await_!(self.0.json_provide_to_x(w))
                } else {
                    let mut w = chained_select_method!(
                        w.start_to_consume_chained_arrays(())
                            .start_to_consume_chained_objects
                            .start_to_consume_chained_strings
                    );
                    () = await_try!(w.extend(&self.0));
                    await_!(w.end_with(&self.1))
                }
            }
        });

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }
});

type CommaSeparated<A, B> = <A as traits::EmptyOrCommaSeparatedElements>::ChainWithComma<B>;
