use alloc::string::String;

use crate::ser::{IntoJson, ToJson2 as ToJson, helpers::json_fns, json_kinds};

impl IntoJson for String {
    type JsonKind = json_kinds::JsonString;
    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        use trait_mod;
        |self, w| {
            use trait_mod::XHelpers;
            self.x_map_ref_1(w, Self::json_provide_to_x)
        }
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl ToJson for String {
    type ToJsonKind = json_kinds::JsonString;
    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        use trait_mod;
        |&self, w| <str as trait_mod::XHelpers>::json_provide_to_x(self, w)
    });
    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
