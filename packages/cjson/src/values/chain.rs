// TODO: optimize IntoJson with IS_CHAINABLE_AND_ALWAYS_EMPTY
use crate::{
    ToJson,
    ser::{
        ConsumeChainedArrays, ConsumeChainedStrings as _, ConsumeJson, Consumed, IntoJson,
        ToJsonArray, ToJsonString, json_kinds, texts,
        traits::{self, Array, EmptyOrCommaSeparatedElements, JsonString},
    },
};

use super::{ChainArray, ChainString};

impl<A: IntoJson<JsonKind = json_kinds::Array>, B: IntoJson<JsonKind = json_kinds::Array>> IntoJson
    for ChainArray<A, B>
{
    type JsonKind = json_kinds::Array;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        let mut w = w.start_to_consume_chained_arrays(());
        w.extend(self.0);
        w.end_with(self.1)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
}
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

impl<A: IntoJson<JsonKind = json_kinds::JsonString>, B: IntoJson<JsonKind = json_kinds::JsonString>>
    IntoJson for ChainString<A, B>
{
    type JsonKind = json_kinds::JsonString;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        let mut w = w.start_to_consume_chained_strings(());
        w.extend(self.0);
        w.end_with(self.1)
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool =
        A::IS_CHAINABLE_AND_ALWAYS_EMPTY && B::IS_CHAINABLE_AND_ALWAYS_EMPTY;
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
