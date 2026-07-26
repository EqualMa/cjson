use alloc::string::String;

use crate::ser::{
    ConsumeJson, Consumed, IntoJson, ToJson, ToJsonString,
    helpers::json_fns,
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
    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, w| {
            use trait_mod::{await_, de_async_move};
            de_async_move!(async move {
                //
                await_!(w.consume_str(&self, ()))
            })
        }
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
