use crate::ser::{IntoJson, ToJson2 as ToJson, helpers::json_fns, json_kinds};

use super::Null;

impl<T: ToJson> ToJson for Option<T> {
    type ToJsonKind = json_kinds::AnyValue;
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        use trait_mod;
        async |&self, w| {
            use trait_mod::{XHelpers as _, await_};
            if let Some(this) = self {
                await_!(w.consume_any_value_of_any_kind(this, ()))
            } else {
                await_!(Null.json_provide_into_x(w))
            }
        }
    });
}

impl<T: IntoJson> IntoJson for Option<T> {
    type JsonKind = json_kinds::AnyValue;
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        async |self, w| {
            use trait_mod::{XHelpers as _, await_};
            if let Some(this) = self {
                await_!(w.consume_any_value_of_any_kind(this, ()))
            } else {
                await_!(Null.json_provide_into_x(w))
            }
        }
    });
}
