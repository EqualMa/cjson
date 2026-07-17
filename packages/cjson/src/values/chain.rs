// TODO: optimize IntoJson with IS_CHAINABLE_AND_ALWAYS_EMPTY
use crate::{
    ToJson,
    ser::{
        ConsumeChainedArrays, ConsumeChainedObjects as _, ConsumeChainedStrings as _, ConsumeJson,
        Consumed, IntoJson, ToJson2, ToJsonArray, ToJsonString, json_kinds, texts,
        traits::{self, Array, EmptyOrCommaSeparatedElements, JsonString},
    },
    utils::impl_many,
};

use super::{ChainArray, ChainString};

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
        }
    }

    impl<A: IntoJson<JsonKind = K>, B: IntoJson<JsonKind = K>> IntoJson for Chain<A, B> {
        type JsonKind = K;

        fn json_provide_into<
            W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<K> = ()>>,
        >(
            self,
            w: W,
        ) -> Consumed<Self::JsonKind, W> {
            if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                self.1.json_provide_into(w)
            } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                self.0.json_provide_into(w)
            } else {
                let mut w = start_to_consume_chained(w);
                w.extend(self.0);
                w.end_with(self.1)
            }
        }

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }

    impl<A: ToJson2<ToJsonKind = K>, B: ToJson2<ToJsonKind = K>> ToJson2 for Chain<A, B> {
        type ToJsonKind = K;

        fn json_provide_to<
            W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<K> = ()>>,
        >(
            &self,
            w: W,
        ) -> Consumed<Self::ToJsonKind, W> {
            if const { A::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                self.1.json_provide_to(w)
            } else if const { B::IS_CHAINABLE_AND_ALWAYS_EMPTY } {
                self.0.json_provide_to(w)
            } else {
                let mut w = start_to_consume_chained(w);
                w.extend(&self.0);
                w.end_with(&self.1)
            }
        }

        const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
            A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
    }
});

impl<A: ToJsonArray, B: ToJsonArray> ToJson for ChainArray<A, B> {
    type ToJson<'a>
        = <Self as ToJsonArray>::ToJsonArray<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_array(self)
    }
}

type CommaSeparated<A, B> = <A as traits::EmptyOrCommaSeparatedElements>::ChainWithComma<B>;

impl<A: ToJsonArray, B: ToJsonArray> ToJsonArray for ChainArray<A, B> {
    type ToJsonArray<'a>
        = texts::Bracketed<
        CommaSeparated<
            <A::ToJsonArray<'a> as traits::Array>::IntoCommaSeparatedElements,
            <B::ToJsonArray<'a> as traits::Array>::IntoCommaSeparatedElements,
        >,
    >
    where
        Self: 'a;

    fn to_json_array(&self) -> Self::ToJsonArray<'_> {
        texts::Bracketed(
            self.0
                .to_json_array()
                .into_comma_separated_elements()
                .chain_with_comma(self.1.to_json_array().into_comma_separated_elements()),
        )
    }
}

impl<A: ToJsonString, B: ToJsonString> ToJson for ChainString<A, B> {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}
impl<A: ToJsonString, B: ToJsonString> ToJsonString for ChainString<A, B> {
    // TODO: optimize
    type ToJsonString<'a>
        = texts::QuotedJsonStringFragment<
        texts::Chain<
            <A::ToJsonString<'a> as traits::JsonString>::IntoJsonStringFragments,
            <B::ToJsonString<'a> as traits::JsonString>::IntoJsonStringFragments,
        >,
    >
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        texts::QuotedJsonStringFragment(texts::Chain(
            self.0.to_json_string().into_json_string_fragments(),
            self.1.to_json_string().into_json_string_fragments(),
        ))
    }
}
