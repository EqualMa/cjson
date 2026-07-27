use crate::ser::{ToJson2, texts};

use super::{helpers::json_fns, json_kinds};

impl ToJson2 for str {
    type ToJsonKind = json_kinds::JsonString;

    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        |&self, w| w.consume_str(self, ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

#[cfg(feature = "alloc")]
mod alloc;

mod r#const;
