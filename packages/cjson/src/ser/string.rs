use crate::ser::{ToJson, ToJson2, ToJsonString, texts};

use super::{helpers::json_fns, json_kinds};

impl ToJson2 for str {
    type ToJsonKind = json_kinds::JsonString;

    json_fns!({
        json_provide_to::json_provide_to_try::json_provide_to_async_try::ToJsonKind;
        |&self, w| w.consume_str(self, ())
    });

    const IS_CHAINABLE_AND_ALWAYS_EMPTY: bool = false;
}

impl ToJson for str {
    type ToJson<'a>
        = <Self as ToJsonString>::ToJsonString<'a>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        Self::to_json_string(self)
    }
}

impl ToJsonString for str {
    type ToJsonString<'a>
        = texts::QuotedJsonStringFragment<texts::StrToJsonStringFragment<'a>>
    where
        Self: 'a;

    fn to_json_string(&self) -> Self::ToJsonString<'_> {
        texts::QuotedJsonStringFragment(texts::StrToJsonStringFragment(self))
    }
}

#[cfg(feature = "alloc")]
mod alloc;

mod r#const;
