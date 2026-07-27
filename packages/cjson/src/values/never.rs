use crate::ser::{IntoJson, ToJsonByCopyIntoJson, helpers::json_fns, json_kinds};

use super::Never;

impl IntoJson for Never {
    type JsonKind = json_kinds::AnyValue; // TODO: NeverJsonKind
    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, _| trait_mod::never_future!(match self {})
    });
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false; // TODO: should this be true or false?
}

impl ToJsonByCopyIntoJson for Never {}
