use crate::ser::{ToJson, ToJson2, ToJsonString, texts};

use super::{ConsumeJson, json_kinds};

impl ToJson2 for str {
    type ToJsonKind = json_kinds::JsonString;

    fn json_provide_to<
        W: ConsumeJson<ConsumeJsonKind: json_kinds::JsonKind<Contains<Self::ToJsonKind> = ()>>,
    >(
        &self,
        w: W,
    ) -> super::Consumed<Self::ToJsonKind, W> {
        w.consume_str(self, ())
    }

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
