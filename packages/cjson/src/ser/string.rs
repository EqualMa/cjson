use crate::ser::{ToJson, ToJsonStringFragment, texts};

impl ToJson for str {
    type ToJson<'a>
        = texts::QuotedJsonStringFragment<texts::StrToJsonStringFragment<'a>>
    where
        Self: 'a;

    fn to_json(&self) -> Self::ToJson<'_> {
        texts::QuotedJsonStringFragment(texts::StrToJsonStringFragment(self))
    }
}

impl ToJsonStringFragment for str {
    type ToJsonStringFragment<'a>
        = texts::StrToJsonStringFragment<'a>
    where
        Self: 'a;

    fn to_json_string_fragment(&self) -> Self::ToJsonStringFragment<'_> {
        texts::StrToJsonStringFragment(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc;

mod r#const;
