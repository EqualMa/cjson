use crate::ser::{ToJson, ToJsonString, texts};

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
