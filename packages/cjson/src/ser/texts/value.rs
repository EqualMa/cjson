use crate::ser::{IntoJson, helpers::json_fns, json_kinds, traits};

use super::Value;

impl<T: traits::IntoTextChunks> IntoJson for Value<T> {
    type JsonKind = json_kinds::AnyValue;

    json_fns!({
        json_provide_into::json_provide_into_try::json_provide_into_async_try::JsonKind;
        |self, w| w.consume_any_value(self, ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}
