use alloc::string::String;

use crate::ser::{ConsumeJson, IntoJson, ToJson2, helpers::json_fns, json_kinds};

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

// TODO: impl ToJson for String
