use alloc::string::String;

use crate::ser::{
    ConsumeJson, Consumed, IntoJson, ToJson, ToJsonString,
    json_kinds::{self, JsonKind},
};

impl ToJson for String {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}

impl ToJsonString for String {
    type ToJsonString<'a>
        = <&'a str as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        str::to_json_string(self)
    }
}

impl IntoJson for String {
    type JsonKind = json_kinds::JsonString;

    fn json_provide_into<
        W: ConsumeJson<ConsumeJsonKind: JsonKind<Contains<Self::JsonKind> = ()>>,
    >(
        self,
        w: W,
    ) -> Consumed<Self::JsonKind, W> {
        w.consume_str(&self, ())
    }

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
